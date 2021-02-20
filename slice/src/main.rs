fn main() 
{
    let mut s = String::from("hello");

    let slice = &s[2..4]; // ll
    let slice = &s[..4]; // hell
    let slice = &s[3..]; // lo

    let slice = first_word_slice(&s);
    s.clear();
    // println!("the first word is: {}", word); // already borrowed as imutable

    let string_literal = "hello world!";
    let word = first_word_slice_for_str(&s[..]);
    let word = first_word_slice_for_str(string_literal);
}

// use slice instead of this!
fn first_word_index(s: &String) -> usize
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' ' 
        {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str 
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() 
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice_for_str(s: &str) -> &str 
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() 
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }

    &s[..]
}