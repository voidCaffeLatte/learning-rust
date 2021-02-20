fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("{}, {}", s, calculate_length(&s));
    }

    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        // let r2 = &mut s; // multiple immutable references or one mutable reference
        r1.push_str("hoge");
    }
}

fn calculate_length(s: &String) -> usize
{
    s.len()
}

fn change(some_string: &mut String)
{
    // must be mutable
    some_string.push_str(", world");
}

// must specify lifetime
// fn dangle() -> &String 
// {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String 
{
    let s = String::from("hello");
    s
}