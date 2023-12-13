mod new_item;

pub use crate::edit::new_item::NewItem;

use crate::index::dict::{EditableIndexDictionary, IndexDictionaryEditor};
use crate::index::posting::{EditableIndexPosting, IndexPostingEditor};
use crate::index::storage::{EditableIndexStorage, IndexStorageEditor, StorageInsertionResult};
use crate::index::traits::editable_index::EditableInvertedIndex;
use crate::Result;
use bytestore::backend::growable::GrowableBackend;
use bytestore::backend::Backend;
use bytestore::traits::deser::Deser;
use fnv::FnvHashMap;
use std::hash::Hash;
use std::io::Write;
use std::marker::PhantomData;
use std::time::Instant;

/// Edits indexes
pub struct IndexEditor<'a, E, B, T, S> {
    index: &'a mut E,

    // Options
    sorted_postings: bool,

    // Temporary insertion data.
    stored_items: Vec<S>,
    terms: FnvHashMap<T, u32>,
    post_map: FnvHashMap<(u16, u32), Vec<u32>>,

    // Temporary term frequency data to reduce insertion time for large indexes
    // term_freq_cache: FnvHashMap<T, u32>,
    term_freq_cache: FnvHashMap<T, u32>,

    p: PhantomData<B>,
}

impl<'a, E, B, T, S> IndexEditor<'a, E, B, T, S> {
    #[inline]
    pub fn new(index: &'a mut E) -> Self {
        Self {
            index,
            sorted_postings: false,
            stored_items: vec![],
            terms: FnvHashMap::default(),
            post_map: FnvHashMap::default(),
            term_freq_cache: FnvHashMap::default(),
            p: PhantomData,
        }
    }

    pub fn with_sorted_postings(mut self) -> Self {
        self.sorted_postings = true;
        self
    }

    #[inline]
    pub fn pending_count(&self) -> usize {
        self.stored_items.len()
    }

    #[inline]
    pub fn has_pending(&self) -> bool {
        self.pending_count() > 0
    }
}

impl<'a, E, B, T, S> IndexEditor<'a, E, B, T, S>
where
    T: Hash + Eq,
    B: Backend,
{
    #[inline]
    pub fn insert(&mut self, new_item: NewItem<T, S>) -> Result<bool> {
        self.insert_in_postings(new_item, &[0])
    }

    pub fn reserve(&mut self, items: usize, terms: usize) {
        self.stored_items.reserve(items);
        self.terms.reserve(terms);
        self.post_map.reserve(terms);
    }

    pub fn insert_in_postings(
        &mut self,
        mut new_item: NewItem<T, S>,
        postings: &[u16],
    ) -> Result<bool> {
        if new_item.terms().is_empty() || postings.is_empty() {
            return Ok(false);
        }

        let temp_term_ids = self.insert_terms_temp(new_item.take_terms());
        let stored_item_id = self.insert_store_item_temp(new_item.into_store_item());

        for posting in postings.iter() {
            self.post_map
                .insert((*posting, stored_item_id), temp_term_ids.clone());
        }

        Ok(true)
    }

    /// Stores the terms temporarily with a temp ID.
    #[inline]
    fn insert_terms_temp(&mut self, terms: Vec<T>) -> Vec<u32> {
        assert!(self.terms.len() + terms.len() <= u32::MAX as usize);

        let mut out = Vec::with_capacity(terms.len());

        for term in terms.into_iter() {
            let len = self.terms.len() as u32;
            let e = self.terms.entry(term).or_insert(len);
            out.push(*e);
        }

        out
    }

    #[inline]
    fn insert_store_item_temp(&mut self, store_item: S) -> u32 {
        let id = self.stored_items.len() as u32;
        self.stored_items.push(store_item);
        id
    }
}

impl<'a, E, B, T, S> IndexEditor<'a, E, B, T, S>
where
    B: GrowableBackend,
    E: EditableInvertedIndex<B, T, S>,
{
    pub fn announce_dict_term_count(&mut self, count: usize, term_avg_size: usize) -> Result<()> {
        let mut dict = self.index.get_dict_mut();
        dict.editor().announce_new_terms(count, term_avg_size)?;
        Ok(())
    }
}

impl<'a, E, B, T, S> IndexEditor<'a, E, B, T, S>
where
    B: GrowableBackend,
    T: Deser + Hash + Eq + Clone,
    S: Deser,
    E: EditableInvertedIndex<B, T, S>,
{
    /// Commits changes into the memory. `finish()` must be called after the last commit!
    pub fn commit(&mut self) -> Result<()> {
        if !self.has_pending() {
            return Ok(());
        }

        print!("Inserting storage items ");
        std::io::stdout().flush().unwrap();
        let start = Instant::now();
        let store_ids = {
            let mut store = self.index.get_storage_mut();
            let mut store_edit = store.editor();
            store_edit.insert_items(&self.stored_items)?
        };
        self.stored_items.clear();
        println!("{:?}", start.elapsed());

        print!("Inserting term ids ");
        std::io::stdout().flush().unwrap();
        let start = Instant::now();
        let mut sum = 0;
        let term_id_map: FnvHashMap<u32, u32> = {
            let mut term_dict = self.index.get_dict_mut();
            let mut term_edit = term_dict.editor();

            let mut term_id_map =
                FnvHashMap::with_capacity_and_hasher(self.terms.len(), Default::default());

            for (term, tmp_id) in self.terms.drain() {
                let new_id = if let Some(new_id) = self.term_freq_cache.get(&term) {
                    *new_id
                } else {
                    let new_id = term_edit.insert_or_get_single(&term)?;
                    self.term_freq_cache.insert(term, new_id);
                    new_id
                };

                term_id_map.insert(tmp_id, new_id);
                sum += 1;
            }

            term_id_map
        };
        println!("{:?} ({sum})", start.elapsed());

        print!("Inserting postings ");
        std::io::stdout().flush().unwrap();
        let mut postings = self.index.get_postings_mut();
        let mut postings_edit = postings.editor();
        let start = Instant::now();

        let mut terms_buf = vec![];

        for ((post_list_id, store_id), terms) in self.post_map.drain() {
            terms_buf.extend(
                terms
                    .iter()
                    .map(|i| term_id_map.get(i).copied().unwrap_or_default() as usize),
            );

            let store_id = match &store_ids {
                StorageInsertionResult::Ids(ids) => ids[store_id as usize],
                StorageInsertionResult::First(first) => *first + store_id as u64,
            };

            postings_edit.insert_posts(post_list_id, store_id, &terms_buf)?;
            terms_buf.clear();
        }
        println!("{:?}", start.elapsed());

        let start = Instant::now();
        postings_edit.commit()?;
        println!("Posting commit: {:?}", start.elapsed());
        println!();

        Ok(())
    }

    /// Finishes editing
    pub fn finish(self) -> Result<()> {
        if self.sorted_postings {
            // TODO: Maybe update only the terms updated in the last commit instead of everything!
            let mut postings = self.index.get_postings_mut();
            let mut postings_edit = postings.editor();
            postings_edit.sort_all_postings()?;
        }
        Ok(())
    }
}
