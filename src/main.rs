/*
use crate::edit::NewItem;
use crate::index::dict::ngram::ngram::Ngram;
use crate::index::preset::{
    CompressedIndex, CompressedIntNgramIndex, CompressedNgramIndex, DefaultIndex,
};
use bytestore::backend::mmap::{MmapBackend, MmapFile};
use bytestore::traits::creatable::Creatable;
use std::process::id;
use std::time::{Duration, Instant};

pub mod edit;
pub mod error;
pub mod index;
pub mod retrieve;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() {
    let backend = MmapBackend::create(MmapFile::create("./mmap_nc_r", 16).unwrap()).unwrap();
    let mut index = CompressedIntNgramIndex::create(backend).unwrap();

    let mut dur = Duration::default();
    let mut editor = index.editor().with_sorted_postings();
    for i in 0..1 {
        for i in 0..1_000_000 {
            let terms = vec![format!("A-{i}"), format!("B-{i}")];
            let terms = terms
                .into_iter()
                .map(|i| {
                    let f = format!("{i:0<10}");
                    Ngram::<10>::try_from(f).unwrap()
                })
                .collect::<Vec<_>>();
            editor.insert(NewItem::new(terms, i)).unwrap();
        }
        let start = Instant::now();
        editor.commit().unwrap();
        dur += start.elapsed();
    }
    let start = Instant::now();
    editor.finish().unwrap();
    let end = start.elapsed();
    println!(
        "Committing 10 million in {:?} ({:?})\nSorting took: {:?}",
        dur,
        id(),
        end
    );
    index.flush().unwrap();
    /*
    let mut backend = MmapBackend::create(MmapFile::create("./mmap_nc", 16).unwrap()).unwrap();
    // let mut backend = MemoryBackend::create(MemoryData::new(vec![0u8; 1000])).unwrap();
    let mut index: TestIndex<_, String, u32> = TestIndex::create(&mut backend).unwrap();


    let backend = MmapFile::load("./mmap_nc").unwrap();
    let backend = MmapBackend::create(backend).unwrap();
    let mut index: TestIndex<_, String, u32> = TestIndex::load(backend).unwrap();
     */

    // println!("{}", backend.free());
    // backend.shrink(10).unwrap();

    /*
    let dict = index.dict();
    let res = dict.term_id(&"A-90".to_string()).unwrap();
    println!("{res}");

    let postings = index.postings();
    let mut retr = postings.posting_retriever(0, res as usize).unwrap();
    for i in retr {
        println!("{i}")
    }
     */
}

/*
pub struct TestIndex<B, T, S> {
    backend: MultiFile<B>,
    p: PhantomData<(T, S)>,
}

impl<B: GrowableBackend, T, S> TestIndex<B, T, S> {
    pub fn create(backend: B) -> Result<Self> {
        let mut mf = MultiFile::create(backend)?;
        mf.insert_new_backend::<DefaultDict<_, String>>()?;
        mf.insert_new_backend::<DefaultStorage<_, usize>>()?;
        // mf.insert_new_backend::<PassThroughStorage<usize>>()?;
        mf.insert_new_backend::<DefaultPostings<_>>()?;
        // mf.insert_new_backend::<CompressedPostings<_>>()?;
        Ok(Self {
            backend: mf,
            p: PhantomData,
        })
    }

    pub fn load(backend: B) -> Result<Self> {
        Ok(Self {
            backend: MultiFile::init(backend)?,
            p: PhantomData,
        })
    }

    #[inline]
    pub fn editor(&mut self) -> IndexEditor<Self, B, T, S> {
        IndexEditor::new(self)
    }

    pub fn postings(&self) -> DefaultPostings<BaseSubBackend<&[u8]>> {
        self.backend.get_backend(2).unwrap()
    }

    pub fn dict(&self) -> DefaultDict<BaseSubBackend<&[u8]>, String> {
        self.backend.get_backend(0).unwrap()
    }

    pub fn flush(&mut self) -> Result<()> {
        self.backend.flush()?;
        Ok(())
    }
}

impl<B, T, S> EditableInvertedIndex<B, T, S> for TestIndex<B, T, S>
where
    B: GrowableBackend,
    T: Deser + Ord + Clone + hashing::Hash + Eq,
    S: Deser + Clone,
{
    type DictImpl<'a>
    = DefaultDict<MFileEntryMut<'a, B>, T>
        where
            B: 'a,
            T: 'a,
            Self: 'a,
    ;

    type StorageImpl<'a>
    = DefaultStorage<MFileEntryMut<'a, B>, S>
    // = PassThroughStorage<S>
        where
            B: 'a,
            S: 'a,
            Self: 'a,
    ;

    type PostingsImpl<'a>
    // = CompressedPostings<MFileEntryMut<'a, B>>
    = DefaultPostings<MFileEntryMut<'a, B>>
        where
            B: 'a,
            Self: 'a,
    ;

    #[inline]
    fn get_dict_mut(&mut self) -> Self::DictImpl<'_> {
        let entry = self.backend.entry_mut(0).unwrap();
        Self::DictImpl::init(entry).unwrap()
    }

    #[inline]
    fn get_storage_mut(&mut self) -> Self::StorageImpl<'_> {
        let entry = self.backend.entry_mut(1).unwrap();
        Self::StorageImpl::init(entry).unwrap()
    }

    #[inline]
    fn get_postings_mut(&mut self) -> Self::PostingsImpl<'_> {
        let entry = self.backend.entry_mut(2).unwrap();
        Self::PostingsImpl::init(entry).unwrap()
    }
}
*/
*/
fn main() {}
