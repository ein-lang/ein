use std::{
    alloc::{alloc, dealloc, Layout},
    ops::Deref,
    ptr::null_mut,
    sync::atomic::{fence, Ordering},
};

const INITIAL_COUNT: usize = 0;

#[derive(Debug)]
pub struct Rc<T> {
    pointer: *const T,
}

struct RcBlock<T> {
    count: std::sync::atomic::AtomicUsize,
    payload: T,
}

impl<T> Rc<T> {
    pub fn new(payload: T) -> Self {
        if Self::is_zero_sized() {
            Self {
                pointer: null_mut(),
            }
        } else {
            let pointer = unsafe { &mut *(alloc(Self::block_layout()) as *mut RcBlock<T>) };

            *pointer = RcBlock::<T> {
                count: std::sync::atomic::AtomicUsize::new(INITIAL_COUNT),
                payload,
            };

            Self {
                pointer: &mut pointer.payload,
            }
        }
    }

    fn block_pointer(&self) -> &RcBlock<T> {
        unsafe { &*((self.pointer as *const usize).offset(-1) as *const RcBlock<T>) }
    }

    fn block_layout() -> Layout {
        Layout::new::<RcBlock<T>>()
    }

    fn is_zero_sized() -> bool {
        Layout::new::<T>().size() == 0
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.block_pointer().payload
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        if !Self::is_zero_sized() {
            // TODO Is this correct ordering?
            self.block_pointer().count.fetch_add(1, Ordering::Relaxed);
        }

        Self {
            pointer: self.pointer,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        if Self::is_zero_sized() {
            return;
        }

        // TODO Is this correct ordering?
        if self.block_pointer().count.fetch_sub(1, Ordering::Release) == INITIAL_COUNT {
            fence(Ordering::Acquire);

            unsafe {
                dealloc(
                    self.block_pointer() as *const RcBlock<T> as *mut u8,
                    Self::block_layout(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        Rc::new(0);
    }

    #[test]
    fn clone() {
        let _ = Rc::new(0);
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
            let _ = Rc::new(());
        }

        #[test]
        #[allow(clippy::unit_cmp)]
        fn load_payload() {
            assert_eq!(*Rc::new(()), ());
        }
    }
}
