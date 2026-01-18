use std::mem;
use std::ops::Deref;

struct RcBox<T> {
    data: T,
    ref_count: u8,
}

struct Rc<T> {
    ptr: *mut RcBox<T>,
}

impl<T> Rc<T> {
    fn new(data: T) -> Self {
        let boxed = Box::new(RcBox { data, ref_count: 1 });
        Rc {
            ptr: Box::into_raw(boxed),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.ptr).ref_count += 1;
        }
        Rc { ptr: self.ptr }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &(*self.ptr).data }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.ptr).ref_count -= 1;
            if (*self.ptr).ref_count == 0 {
                drop(Box::from_raw(self.ptr));
            }
        }
    }
}

pub fn memory_leak() {
    println!("--- without forget ---");
    let rc = Rc::new(Box::new(42));
    for i in 1..=3 {
        drop(rc.clone());
        println!("{} clones: ref_count: {}", i, unsafe {
            (*rc.ptr).ref_count
        });
    }

    println!("\n--- with forget ---");
    let rc = Rc::new(Box::new(42));
    for i in 1..=3 {
        mem::forget(rc.clone());
        println!("{} clones: ref_count: {}", i, unsafe {
            (*rc.ptr).ref_count
        });
    }
}

pub fn use_after_free() {
    let rc = Rc::new(Box::new(42));
    for i in 1..=255 {
        mem::forget(rc.clone());
        if matches!(i, 1 | 2 | 254 | 255) {
            println!("{:>3} clones: ref_count: {:>3}", i, unsafe {
                (*rc.ptr).ref_count
            });
        }
    }
    drop(rc.clone());
    println!("after drop: ref_count: {:>3}", unsafe {
        (*rc.ptr).ref_count
    });
}
