use std::alloc::{alloc, handle_alloc_error, Layout};
use std::any::TypeId;
use std::collections::LinkedList;
use std::num::NonZeroUsize;
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub(crate) struct TypeMetadata {
    id: TypeId,
    size: usize,
    align: usize,
}

pub(crate) struct AttributeDataArray {
    type_metadata: TypeMetadata,
    address: NonNull<u8>,
}

impl AttributeDataArray {
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

pub(crate) struct AttributeDataMap {
    capacity: NonZeroUsize,
    length: usize,
    unused_indexes: LinkedList<usize>,
    data: Vec<AttributeDataArray>,
}

pub(crate) struct Application {
    universe_data: Vec<AttributeDataMap>,
}
