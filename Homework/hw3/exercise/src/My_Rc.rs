use std::cell::Cell;
use std::ops::Deref;
use core::ptr::{self, NonNull};
use core::fmt;
use std::marker::PhantomData;
// 实现简易的引用计数指针MyRc,类似于std::rc::RC
struct RcBox<T> {
    value: T,
    strong: Cell<usize>,
}
impl<T> RcBox<T> {
    pub fn new(value: T) -> RcBox<T> {
        RcBox {
            value,
            strong: Cell::new(1),
        }
    }
    fn inc_strong(&self) {
        let strong = self.strong.get();
        self.strong.set(strong + 1);
    }
    fn dec_strong(&self) {
        let strong = self.strong.get();
        self.strong.set(strong - 1);
    }
    fn ret_value(&self) -> &T {
        &self.value
    }
    fn ret_strong(&self) -> usize {
        self.strong.get()
    }
}
pub struct MyRc<T> {
    ptr: NonNull<RcBox<T>>,
    phantom: PhantomData<RcBox<T>>,
}
impl<T> MyRc<T> {
    fn inner(&self) -> &RcBox<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn new(value: T) -> MyRc<T> {
        unsafe {
            Self::from_inner(
                Box::leak(Box::new(RcBox {
                    value,
                    strong: Cell::new(1),
                })).into(),
            )
        }
    }

    unsafe fn from_inner(ptr: NonNull<RcBox<T>>) -> Self {
        Self {
            ptr,
            phantom: PhantomData,
        }
    }
    pub unsafe fn get_mut_unchecked(this: &mut Self) -> &mut T {
        unsafe{ &mut (*this.ptr.as_ptr()).value }
    }
    pub fn get_value(&self) -> &T {
        self.inner().ret_value()
    }
    pub fn get_strong(&self) -> usize {
        self.inner().ret_strong()
    }
}
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner().value
    }
}
impl<T> Clone for MyRc<T> {
    fn clone(&self) -> MyRc<T> {
        unsafe {
            self.inner().inc_strong();
            Self::from_inner(self.ptr)
        }
    }
}
impl<T: fmt::Display> fmt::Display for MyRc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}
impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            self.inner().dec_strong();
            if self.inner().strong.get() == 0 {
                println!("dropped!");
                ptr::drop_in_place(Self::get_mut_unchecked(self));
            }
        }
    }
}