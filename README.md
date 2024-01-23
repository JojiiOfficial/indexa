# Indexa
Rust framework to create highly efficient in memory or memmapped inverted indexes.
This is built on using components provided by the [`bytestore`](https://github.com/JojiiOfficial/Bytestore) crate.

# Example
```rust
use bytestore::{
    backend::mmap_mut::{MmapBackendMut, MmapFileMut},
    traits::creatable::Creatable,
};
use indexa::{
    edit::NewItem,
    index::{preset::CompressedIndex, storage::IndexStorage},
    retrieve::{build::RetrieverBuilder, retriever::default::DefaultRetriever},
};

fn main() {
    // Create a memory mapped backend. See the [`bytestore`] crate for more details!
    let backend =
        MmapBackendMut::create(MmapFileMut::create("./index", 1024 * 1024).unwrap()).unwrap();

    // Create a new index in the backend.
    let mut index: CompressedIndex<_, String, String> = CompressedIndex::create(backend).unwrap();

    // Creates an editor for the index. An editor holds the inserted elements in memory and inserts
    // them on commit(). This allows efficient insertion of the indexed data into a
    // byte-array/memmapped file. If you want to insert more data that can fit in your RAM use
    // commit to clear up the memory.
    // Sorting the postings can take some time but allows faster retrieveal.
    let mut editor = index.editor().with_sorted_postings();
    let item = NewItem::new(
        vec!["hello".to_string(), "world".to_string()],
        "Hello world!".to_string(),
    );

    editor.insert(item).unwrap();
    let item = NewItem::new(
        vec!["hello".to_string(), "you".to_string()],
        "Hello you!".to_string(),
    );
    editor.insert(item).unwrap();

    editor.commit().unwrap();
    editor.finish().unwrap();

    let mut retriever_builder = RetrieverBuilder::new(&index);
    retriever_builder.add_term(&String::from("hello")).unwrap();

    let retriever: DefaultRetriever<_> = retriever_builder.retriever();
    for storage_id in retriever {
        let item = index.storage().get_item(storage_id as usize).unwrap();
        println!("{item}");
    }

    // Output:
    // Hello world!
    // Hello you!
}

```
