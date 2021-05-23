use std::{
    alloc::{alloc, dealloc, Layout},
    mem::transmute,
    ops::Deref,
    ptr::null_mut,
    sync::atomic::Ordering,
};

const INITIAL_COUNT: usize = 0;

#[derive(Debug)]
pub struct Rc<T> {
    pointer: *mut T,
}

struct RcBlock<T> {
    count: std::sync::atomic::AtomicUsize,
    payload: T,
}

impl<T> Rc<T> {
    pub fn new(payload: T) -> Self {
        if Self::block_layout().size() == 0 {
            Self {
                pointer: null_mut(),
            }
        } else {
            let pointer = unsafe { transmute::<_, &mut RcBlock<T>>(alloc(Self::block_layout())) };

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
        unsafe {
            transmute::<_, &mut RcBlock<T>>(transmute::<_, *mut usize>(self.pointer).offset(-1))
        }
    }

    fn block_layout() -> Layout {
        Layout::new::<RcBlock<T>>()
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
        // TODO Optimize the ordering.
        self.block_pointer().count.fetch_add(1, Ordering::SeqCst);

        Self {
            pointer: self.pointer,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let count = self.block_pointer().count.fetch_sub(1, Ordering::SeqCst);

        if count == INITIAL_COUNT {
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
        let _ = Rc::new(0).clone();
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
            let _ = Rc::new(()).clone();
        }

        #[test]
        fn load_payload() {
            assert_eq!(*Rc::new(()), ());
        }
    }
}
