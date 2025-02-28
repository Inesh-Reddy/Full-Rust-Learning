pub fn explain_mutability() {
    println!("🔍 Mutability Example in Rust");
    let x  = String::from("Immutable data doesnot give any issues..");
    // x.push_str("I cannot be changed");//gives  error
    println!("{}", x);

    let mut y = String::from("Hey I am mutable, you can change me...");
    y.push_str("They are changing me....");
    println!("{}",y);

}
