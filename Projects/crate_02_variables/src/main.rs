fn booleans(){
    let is_male = true;
    let is_major = true;
    if is_male {
        println!("4 . You are Male");
    } else {
        println!("5 . You are not Male");
    }

    if is_male && is_major {
        println!("6 . you can vote!!");
    }

}
fn main() {
    println!("-------------");
    println!(" Variables : ");
    println!("-------------");
    println!();
    let x = -10;
    println!("1 . Signed integer variable x is assigned to a value   : {}", x);
    let y: u32 = 1000;
    println!("2 . UnSigned integer variable y is assigned to a value : {}", y);
    let z: f32 = 100.00;
    println!("3 . Float variable z is assigned to a value            : {}", z);
    booleans();
}
