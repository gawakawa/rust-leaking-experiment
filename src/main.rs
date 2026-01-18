use rust_leaking_experiment::{drain, rc};

fn main() {
    println!("=== drain ===");
    drain::drain_leak();
    println!("\n=== Rc memory leak ===");
    rc::memory_leak();
    println!("\n=== Rc use after free ===");
    rc::use_after_free();
}
