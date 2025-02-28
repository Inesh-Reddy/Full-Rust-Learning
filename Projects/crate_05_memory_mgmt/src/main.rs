use crate_05_memory_mgmt::stack_heap;
use crate_05_memory_mgmt::borrowing;
use crate_05_memory_mgmt::mutability;
use crate_05_memory_mgmt::ownership;

fn main() {
    stack_heap::explain_stack_heap();
    stack_heap::explain_mutability();
    stack_heap::explain_ownership();
    stack_heap::explain_borrowing();
    println!("Hello, world!");
}
