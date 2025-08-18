use std::alloc::{Layout, alloc, handle_alloc_error};
use std::collections::LinkedList;
use std::num::NonZeroUsize;
use std::ptr::NonNull;
use utils::reflect::TypeMetadata;

#[derive(Clone, PartialEq, Eq)]
pub struct Name(u128);

impl Name {
    pub const fn from_str(s: &str) -> Self {
        Self(utils::hash128(s))
    }
}

pub(crate) struct DataArray {
    type_metadata: TypeMetadata,
    address: NonNull<u8>,
}

impl DataArray {
    pub fn new(type_metadata: TypeMetadata, capacity: NonZeroUsize) -> Self {
        let address = unsafe {
            let layout = Layout::from_size_align_unchecked(
                type_metadata.size * capacity.get(),
                type_metadata.align,
            );
            let ptr = alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            NonNull::new_unchecked(ptr)
        };
        Self {
            type_metadata,
            address,
        }
    }
}

pub(crate) struct DataMapInner {
    fields_data: Vec<(Name, DataArray)>,
    children_data: Vec<(Name, DataMapInner)>,
}

pub(crate) struct DataMap {
    capacity: NonZeroUsize,
    length: usize,
    unused_indexes: LinkedList<usize>,
    inner: DataMapInner,
}
