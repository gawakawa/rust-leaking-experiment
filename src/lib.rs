use std::mem;

pub fn drain_leak() {
    let mut vec = vec![Box::new(0); 4];

    let ptr: *const i32 = &*vec[0];
    println!("addr: {:p}, value: {}", ptr, unsafe { *ptr });

    {
        let mut drainer = vec.drain(..);

        drainer.next();
        drainer.next();

        mem::forget(drainer);
    }

    println!("addr: {:p}, value: {}", ptr, unsafe { *ptr });
}
