fn do_sum(a: i32, b: i32) -> i32 {
    return a+b;
}
fn main() {
    println!("Hello, world!");
    let a = 10;
    let b = 20;
    let c = do_sum(a, b);
    println!("Sum of {} & {} : {}", a,b,c);
}
