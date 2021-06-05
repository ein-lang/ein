use super::arc_block::ArcBlock;
use std::{
    alloc::Layout,
    intrinsics::copy_nonoverlapping,
    ptr::{null, null_mut},
};

#[derive(Debug)]
#[repr(C)]
pub struct ArcBuffer {
    block: ArcBlock,
}

#[repr(C)]
struct ArcBufferInner {
    length: usize,
    first_byte: u8,
}

impl ArcBuffer {
    pub fn new(length: usize) -> Self {
        Self {
            block: if length == 0 {
                ArcBlock::null()
            } else {
                let mut block = ArcBlock::new(
                    Layout::new::<usize>()
                        .extend(Layout::array::<u8>(length).unwrap())
                        .unwrap()
                        .0
                        .pad_to_align(),
                );

                unsafe { *(block.ptr_mut() as *mut usize) = length }

                block
            },
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            if self.block.is_null() {
                std::slice::from_raw_parts(null(), 0)
            } else {
                let inner = &*(self.block.ptr() as *const ArcBufferInner);

                std::slice::from_raw_parts(&inner.first_byte, inner.length)
            }
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            if self.block.is_null() {
                std::slice::from_raw_parts_mut(null_mut(), 0)
            } else {
                let inner = &mut *(self.block.ptr() as *mut ArcBufferInner);

                std::slice::from_raw_parts_mut(&mut inner.first_byte, inner.length)
            }
        }
    }
}

impl Clone for ArcBuffer {
    fn clone(&self) -> Self {
        Self {
            block: self.block.clone(),
        }
    }
}

impl Drop for ArcBuffer {
    fn drop(&mut self) {
        self.block.drop::<()>();
    }
}

impl From<&[u8]> for ArcBuffer {
    fn from(slice: &[u8]) -> Self {
        let mut buffer = Self::new(slice.len());

        unsafe {
            copy_nonoverlapping(
                slice.as_ptr(),
                buffer.as_slice_mut().as_mut_ptr(),
                slice.len(),
            )
        }

        buffer
    }
}

impl From<Vec<u8>> for ArcBuffer {
    fn from(vec: Vec<u8>) -> Self {
        vec.as_slice().into()
    }
}

impl From<&str> for ArcBuffer {
    fn from(vec: &str) -> Self {
        vec.as_bytes().into()
    }
}

impl From<String> for ArcBuffer {
    fn from(string: String) -> Self {
        Vec::<u8>::from(string).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drop<T>(_: T) {}

    #[test]
    fn create_buffer() {
        ArcBuffer::new(42);
    }

    #[test]
    fn create_zero_sized_buffer() {
        ArcBuffer::new(0);
    }

    #[test]
    fn clone() {
        let arc = ArcBuffer::new(42);
        drop(arc.clone());
        drop(arc);
    }

    #[test]
    fn convert_from_vec() {
        ArcBuffer::from(vec![0u8; 42]);
    }

    #[test]
    fn convert_from_string() {
        ArcBuffer::from("hello".to_string());
    }
}
