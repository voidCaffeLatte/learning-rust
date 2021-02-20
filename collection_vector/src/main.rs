fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // let does_not_exist = &v[100]; // panic!
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(0); // Already borrowed as immutable!
    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v
    {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v
    {
        *i += 50;
    }

    let row = vec![
        SpreadsheelCell::Int(3),
        SpreadsheelCell::Text(String::from("blue")),
        SpreadsheelCell::Float(10.12)
    ];
}

enum SpreadsheelCell
{
    Int(i32),
    Float(f64),
    Text(String),
}