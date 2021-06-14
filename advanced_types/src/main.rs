type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));

    
}

type Thunk = Box<Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    // do something
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

fn generic_sized<T: Sized>(t: T) // "Sized" added automatically
{ 
    // --snip--
}

fn generic_unsized<T: ?Sized>(t: &T) 
{
    // --snip--
}