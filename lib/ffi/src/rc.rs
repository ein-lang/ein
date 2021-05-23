use std::{
    alloc::{alloc, dealloc, Layout},
    intrinsics::copy_nonoverlapping,
    ops::Deref,
    ptr::null,
    sync::atomic::{fence, AtomicUsize, Ordering},
};

const INITIAL_COUNT: usize = 0;

#[derive(Debug)]
pub struct Rc<T> {
    pointer: *const T,
}

struct RcInner<T> {
    count: AtomicUsize,
    payload: T,
}

impl<T> Rc<T> {
    pub fn new(payload: T) -> Self {
        if Layout::new::<T>().size() == 0 {
            Self { pointer: null() }
        } else {
            let pointer = unsafe { &mut *(alloc(Self::inner_layout()) as *mut RcInner<T>) };

            *pointer = RcInner::<T> {
                count: AtomicUsize::new(INITIAL_COUNT),
                payload,
            };

            Self {
                pointer: &pointer.payload,
            }
        }
    }

    fn inner(&self) -> &RcInner<T> {
        unsafe { &*((self.pointer as *const usize).offset(-1) as *const RcInner<T>) }
    }

    fn inner_mut(&self) -> &mut RcInner<T> {
        unsafe { &mut *((self.pointer as *const usize).offset(-1) as *mut RcInner<T>) }
    }

    fn is_pointer_null(&self) -> bool {
        self.pointer == null()
    }

    fn inner_layout() -> Layout {
        Layout::new::<RcInner<T>>()
    }
}

impl Rc<u8> {
    fn buffer(length: usize) -> Self {
        if length == 0 {
            Self { pointer: null() }
        } else {
            let pointer = unsafe {
                &mut *(alloc(Layout::from_size_align(length, 1).unwrap()) as *mut RcInner<u8>)
            };

            pointer.count = AtomicUsize::new(INITIAL_COUNT);

            Self {
                pointer: &pointer.payload,
            }
        }
    }

    fn pointer_mut(&self) -> *mut u8 {
        &mut self.inner_mut().payload as *mut u8
    }
}

impl From<Vec<u8>> for Rc<u8> {
    fn from(vec: Vec<u8>) -> Self {
        let rc = Self::buffer(vec.len());

        unsafe { copy_nonoverlapping(vec.as_ptr(), rc.pointer_mut(), vec.len()) }

        rc
    }
}

impl From<String> for Rc<u8> {
    fn from(string: String) -> Self {
        Vec::<u8>::from(string).into()
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner().payload
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        if !self.is_pointer_null() {
            // TODO Is this correct ordering?
            self.inner().count.fetch_add(1, Ordering::Relaxed);
        }

        Self {
            pointer: self.pointer,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        if self.is_pointer_null() {
            return;
        }

        // TODO Is this correct ordering?
        if self.inner().count.fetch_sub(1, Ordering::Release) == INITIAL_COUNT {
            fence(Ordering::Acquire);

            unsafe {
                // TODO This is invalid for Rc<u8> buffer.
                dealloc(
                    self.inner() as *const RcInner<T> as *mut u8,
                    Self::inner_layout(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drop<T>(_: T) {}

    #[test]
    fn create() {
        Rc::new(0);
    }

    #[test]
    fn clone() {
        let rc = Rc::new(0);
        drop(rc.clone());
        drop(rc);
    }

    #[test]
    fn load_payload() {
        assert_eq!(*Rc::new(42), 42);
    }

    mod zero_sized {
        use super::*;

        #[test]
        fn create() {
            Rc::new(());
        }

        #[test]
        fn clone() {
            let rc = Rc::new(());
            drop(rc.clone());
            drop(rc);
        }

        #[test]
        #[allow(clippy::unit_cmp)]
        fn load_payload() {
            assert_eq!(*Rc::new(()), ());
        }
    }

    mod buffer {
        use super::*;

        #[test]
        fn create_buffer() {
            Rc::buffer(42);
        }

        #[test]
        fn create_zero_sized_buffer() {
            Rc::buffer(0);
        }

        #[test]
        fn clone() {
            let rc = Rc::buffer(42);
            drop(rc.clone());
            drop(rc);
        }

        #[test]
        fn convert_from_vec() {
            Rc::<u8>::from(vec![0u8; 42]);
        }

        #[test]
        fn convert_from_string() {
            Rc::<u8>::from("hello".to_string());
        }
    }
}
