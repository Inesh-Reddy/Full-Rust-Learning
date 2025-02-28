Rust Learning Path

🌱 1. Why Rust Was Created?

Rust was designed to offer memory safety, concurrency, and performance without a garbage collector.

Inspired by C/C++, but eliminates common issues like null pointers, data races, and buffer overflows.

Used in systems programming, blockchain, game development, and web assembly.

🔹 2. Building Blocks of Rust

Memory Management (Stack & Heap, Ownership, Borrowing, Lifetimes)

Primitives (Integers, Floats, Booleans, Characters, Tuples, Arrays)

Strings (String, &str, String::from, Mutability)

Functions (Normal, Closures, Function Pointers)

Control Flow (if/else, match, loops)

Structs & Enums

Traits & Generics

Concurrency (Threads, Channels, Async/Await)

📌 3. Rust Memory Management

3.1 Stack vs Heap

Stack: Fixed-size, fast, used for local variables.

Heap: Dynamic allocation, slower, used for large or dynamic data.

3.2 Ownership Rules

Each value has a single owner.

Ownership is transferred on assignment.

When an owner goes out of scope, value is dropped.

3.3 Borrowing & References

Immutable Borrowing: &String (Read-only access)

Mutable Borrowing: &mut String (One at a time, avoids data races)

Difference from References: Borrowing follows ownership rules, references do not create a new owner.

3.4 Lifetimes

Prevent dangling references.

Defined using 'a (e.g., fn longest<'a>(x: &'a str, y: &'a str) -> &'a str)

🛠 4. Primitive Data Types

4.1 Scalar Types

Integer: i8, i16, i32, i64, i128, isize

Float: f32, f64

Boolean: bool

Character: char

4.2 Compound Types

Tuple: (i32, f64, bool)

Array: [i32; 5]

📝 5. Strings in Rust

5.1 String Types

String (String) – Growable, stored on the heap.

String Slice (&str) – Immutable, borrowed reference.

5.2 Creating Strings

let s1 = String::from("Hello");
let s2 = "Hello".to_string();
let s3 = "Hello"; // &str

5.3 Getting Characters from a String

let greeting = String::from("Hello, Rust!");
let char1 = greeting.chars().nth(4);

match char1 {
    Some(c) => println!("Character: {}", c),
    None => println!("No character found"),
}

🔗 6. Parsing Data (JSON & HTTP Requests)

6.1 Using serde_json for JSON Parsing

use serde_json::Value;

let json_data = r#"{"name": "Alice", "age": 25}"#;
let parsed: Value = serde_json::from_str(json_data).unwrap();
println!("Name: {}", parsed["name"]);

6.2 Making an HTTP Request with reqwest

use reqwest;

#[tokio::main]
async fn main() {
    let response = reqwest::get("https://api.github.com").await.unwrap().text().await.unwrap();
    println!("Response: {}", response);
}

🏗 7. Structs, Enums, and Traits

7.1 Structs

struct User {
    name: String,
    age: u32,
}

7.2 Enums

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

7.3 Traits (Like Interfaces in OOP)

trait Speak {
    fn say_hello(&self);
}

struct Person;

impl Speak for Person {
    fn say_hello(&self) {
        println!("Hello, Rust!");
    }
}

⚡ 8. Concurrency & Async in Rust

8.1 Using Threads

use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Thread running...");
    });
    handle.join().unwrap();
}

8.2 Async/Await (With tokio)

use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    sleep(Duration::from_secs(1)).await;
    println!("Async Done!");
}

🔥 9. Error Handling

9.1 Using Result<T, E>

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

9.2 Using ? Operator

fn read_file() -> std::io::Result<String> {
    std::fs::read_to_string("file.txt")
}



