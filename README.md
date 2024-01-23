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

    // Commit the inserted items and write them into the index.
    editor.commit().unwrap();
    // Closes the editor, applying post some processings such a sorting postings.
    editor.finish().unwrap();

    // Build an index retriever using a term.
    let mut retriever_builder = RetrieverBuilder::new(&index);
    retriever_builder.add_term(&String::from("hello")).unwrap();

    // The retrieving algorithm itself is generic. The `DefaultRetriever` yields all storage IDs
    // that have at least one of the terms assigned.
    //
    // There are other retrieving algorithms that require all terms to be present - which need
    // sorted postings.
    let retriever: DefaultRetriever<_> = retriever_builder.retriever();
    for storage_id in retriever {
        // Get the item from the indexes storage
        let item = index.storage().get_item(storage_id as usize).unwrap();
        println!("{item}");
    }

    // Output:
    // Hello world!
    // Hello you!
}
```

# Presets
| Name | Description |
| ----------- | ----------- |
| DefaultIndex | Normal inverted index implementation without any special features. |
| CompressedIndex | Inverted index with compressed posting lists. Reduces the filesize for larger indexes with a light overhead when retrievig. |
| CompressedIntIndex | Similar to CompressedIndex but dosen't store anything in the indexes 'storage' but rather uses the provided IDs when indexing. Can be useful if the actual data is not stored within the index itself. |
| DefaultNgramIndex | Similar to DefaultIndex but uses NGram (or bytegrams) as index terms. Can be used if the indexed terms all have the same length. Reduces size of the index a lot. |
| CompressedNgramIndex | Similar to CompressedIndex but made for Ngrams. |
| CompressedIntNgramIndex | Similar to CompressedIntIndex but bade for Ngrams. |
