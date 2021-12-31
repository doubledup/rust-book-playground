fn main() {
    let s = String::from("foo bar bundy");
    let word = String::from(&s[..first_word_usize(&s)]);
    println!("First word of '{}': {}", s, word);

    let t = "hello world";
    let hello = &t[..5];
    let world = &t[6..11];
    println!("hello string: '{}'", hello);
    println!("world string: '{}'", world);

    let word = first_word_slice(&s);
    println!("First word of '{}': {}", s, word);

    let mut greeting = String::from("hello world");
    let word = first_word_usize(&greeting);
    greeting.clear();
    println!("the first word is: {}", word);

    // let arr = [1, 2, 3, 4, 5];
    // let slice = &arr[1..3];
}

fn first_word_usize(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
