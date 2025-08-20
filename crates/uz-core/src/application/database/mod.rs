use std::alloc::{alloc, handle_alloc_error, Layout};
use std::collections::LinkedList;
use std::num::NonZeroUsize;
use std::ptr::NonNull;
use utils::reflect::TypeMetadata;

pub(in crate::application) struct ColumnData {
    type_metadata: TypeMetadata,
    address: NonNull<u8>,
}

impl ColumnData {
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

pub(in crate::application) struct TableData {
    capacity: NonZeroUsize,
    length: usize,
    unused_indexes: LinkedList<usize>,
    columns_data: Vec<ColumnData>,
}

pub(in crate::application) struct Database {
    tables: Vec<TableData>,
}

pub(crate) struct TableID(usize);
pub(crate) struct ColumnID(usize);
pub(crate) struct RowID(usize);
