use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct DictItem<T> {
    item: T,
    id: u32,
}

impl<T> DictItem<T> {
    #[inline]
    pub fn new(item: T, id: u32) -> Self {
        Self { item, id }
    }

    #[inline]
    pub fn item(&self) -> &T {
        &self.item
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.item
    }
}
