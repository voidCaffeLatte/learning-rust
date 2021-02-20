fn main() {
    let x = 5;
    let y = x;

    println!("the value of x is: {}", x);

    {
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}, world", s1); // value of s1 is already moved
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("{}, world", s1);
    }

    {
        let a1 = [1, 2, 3, 4 ,5];
        let a2 = a1;
        println!("first value of a1: {}", a1[0]);
    }

    {
        let a1 = [String::from("hello"), String::from("world"), String::from("!")];
        let a2 = a1;
        // println!("first value of a1: {}", a1[0]); // value of a1 is already moved
    }

    {
        let s1 = "nice";
        let s2 = s1;
        println!("{}", s1);
    }

    {
        let s = String::from("hello");
        take_ownership(s);
        let x = 5;
        makes_copy(x);
        // println!("{}", s); // value of s is already moved
    }

    {
        let s1 = gives_ownership();
        println!("{}", s1);

        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        // println!("{}", s2); // value of s2 is already moved
        println!("{}", s3);
    }
}

fn take_ownership(some_string: String)
{
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32)
{
    println!("{}", some_integer);
}

fn gives_ownership() -> String 
{
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String
{
    a_string
}