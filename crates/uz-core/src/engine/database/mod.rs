use std::alloc;
use std::collections::LinkedList;
use std::num::NonZeroUsize;
use std::ptr::NonNull;

pub struct TableData<T> {
    head_pointer: NonNull<T>,
    capacity: NonZeroUsize,
    length: usize,
    unused_indexes: LinkedList<usize>,
}

impl<T> TableData<T> {
    pub fn new(capacity: NonZeroUsize) -> Result<Self, alloc::LayoutError> {
        let head_pointer = unsafe {
            NonNull::<T>::new_unchecked(
                alloc::alloc(alloc::Layout::array::<T>(capacity.get())?) as *mut T
            )
        };
        Ok(Self {
            head_pointer,
            capacity,
            length: 0,
            unused_indexes: LinkedList::new(),
        })
    }

    pub fn add(&mut self, value: T) -> usize {
        todo!()
    }
}

pub trait Database {}
