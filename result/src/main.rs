use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() 
{
    open_file_with_expect();
    
    // let f = File::open("hello.txt")?; // Can NOT do this in main function
}

fn open_file_with_match()
{
    let f = File::open("hello.txt");
    let f = match f
    {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => 
        {
            match File::create("hello.txt")
            {
                Ok(fc) => fc,
                Err(e) => 
                {
                    panic!("Tried to create file but there was a problem: {:?}", e);
                }
            }
        },
        Err(error) =>
        {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
}

fn open_file_with_unwrap()
{
    let f = File::open("hello.txt").unwrap(); // return T if Ok(T), call panic if Err(E)
}

fn open_file_with_expect()
{
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // return T if Ok(T), call panic with message if Err(E)
}

fn read_username_from_file() -> Result<String, io::Error>
{
    let f = File::open("hello.txt");
    let mut f = match f
    {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s)
    {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_username_from_file_2() -> Result<String, io::Error>
{
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}