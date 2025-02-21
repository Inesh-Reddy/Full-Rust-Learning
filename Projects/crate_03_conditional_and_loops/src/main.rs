fn ifelse(){
    let is_even = true;

    if is_even {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }
}

fn get_first_word(sentence: String) -> String {
    let mut resultval = String::new();
    for i in sentence.chars() {
        if i != ' ' {
            resultval = format!("{}{}", resultval, i);
        } else {
            return resultval;
        }
    }
    resultval
}

fn loops(){
     let sentence = String::from("my name is baba voss");
     let first_word = get_first_word(sentence);
     println!("First wordis : {}", first_word);
}

fn main() {
    println!("Hello, world!");
    ifelse();
    loops();
}
