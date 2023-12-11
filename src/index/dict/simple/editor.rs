use crate::index::dict::simple::item::DictItem;
use crate::index::dict::simple::SimpleDict;
use crate::index::dict::{EditIndexDictionary, IndexDictionary};
use crate::traits::deser::Deser;
use crate::Result;
use mapstore::backend::growable::GrowableBackend;
use mapstore::backend::Backend;
use std::borrow::Borrow;

/// Edits SimpleDicts and allows insertion of new terms.
pub struct DictEditor<'a, B, T> {
    dict: &'a mut SimpleDict<B, T>,

    // Temp data
    id_cache: Vec<IdRef>,
    insert_cache: Vec<Vec<u8>>,
}

impl<'a, B, T> DictEditor<'a, B, T> {
    #[inline]
    pub(crate) fn new(dict: &'a mut SimpleDict<B, T>) -> Self {
        Self {
            dict,
            id_cache: vec![],
            insert_cache: vec![],
        }
    }
}

/// Inserts a new `item` with DictItem<T> encoded in `data` into the given `dict` returning the ID.
/// Asserts that there is no such T in the dictionary.
fn insert_new_item<B: GrowableBackend, T: Ord + Deser>(
    dict: &mut SimpleDict<B, T>,
    item: &T,
    data: &[u8],
) -> Result<u32> {
    let pos = match dict.binary_search(item) {
        Err(id) => id,
        Ok(id) => {
            return Ok(dict.dict_item_at(id).unwrap().id());
        }
    };
    let id = dict.backend.count();
    dict.backend.insert_at(data, pos)?;
    Ok(id as u32)
}

impl<'a, B, T> EditIndexDictionary<T> for DictEditor<'a, B, T>
where
    B: GrowableBackend,
    T: Deser + Ord,
{
    fn insert_or_get<I: IntoIterator<Item = T>>(&mut self, terms: I) -> Result<Vec<u32>> {
        self.id_cache.clear();
        self.insert_cache.clear();

        let mut missing_terms = vec![];

        for (pos, term) in terms.into_iter().enumerate() {
            match self.dict.term_id(&term) {
                Some(id) => self.id_cache.push(IdRef::Existing(id)),
                None => {
                    self.id_cache.push(IdRef::New(pos as u32));
                    let id = missing_terms.len() + self.dict.backend.count();
                    let item: DictItem<&T> = DictItem::new(&term, id as u32);
                    self.insert_cache.push(bitcode::serialize(&item)?);
                    missing_terms.push(term);
                }
            }
        }

        let new_entry_count = self.insert_cache.len();
        if new_entry_count > 0 {
            let new_data_len: usize = self.insert_cache.iter().map(|i| i.len()).sum();
            self.dict.backend.grow(new_entry_count, new_data_len)?;
        }

        self.id_cache
            .iter()
            .map(|idref| match idref {
                IdRef::Existing(e) => return Ok(*e),
                IdRef::New(n) => insert_new_item(
                    &mut self.dict,
                    &missing_terms[*n as usize],
                    &self.insert_cache[*n as usize],
                ),
            })
            .collect()
    }
}

enum IdRef {
    Existing(u32),
    New(u32),
}
