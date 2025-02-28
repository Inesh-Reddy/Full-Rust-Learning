use crate_05_memory_mgmt::stack_heap;
use crate_05_memory_mgmt::borrowing;
use crate_05_memory_mgmt::mutability;
use crate_05_memory_mgmt::ownership;

fn main() {
    println!("Hello, World of Rust...");
    println!("Stack and Heap");
    println!("----------------");
    stack_heap::explain_stack_heap();
    stack_heap::heap_fn();
    stack_heap::stack_fn();
    stack_heap::update_string();
        println!("----------------");
    mutability::explain_mutability();
    ownership::explain_ownership();
    borrowing::explain_borrowing();
    
}
