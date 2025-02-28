pub fn explain_stack_heap() {
    println!("🔍 Stack & Heap Example in Rust");
}

pub fn stack_fn(){
    let a = 10;
    let b = 20;
    let c = a+b;
    println!("Stack function: The Sum of {} and {} is {}", a,b,c);
}

pub fn heap_fn(){
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap Function: Final string is {}", combined);
}

pub fn update_string(){
    let mut s = String::from("initial string");
    println!("Before update: {}", s);

    s.push_str("....adding some additional text");
    println!("After update: {}", s);
}
