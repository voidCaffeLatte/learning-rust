use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe
    {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe
    {
        dangerous();
    }

    unsafe
    {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    add_to_counter(3);
    unsafe
    {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous()
{
    // do something
}

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32])
// {
//     let len = slice.len();
//     assert!(mid <= len);
//     (&mut slice[..mid], &mut slice[mid..])
// }

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32])
{
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe
    {
        (slice::from_raw_parts_mut(ptr, mid), slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

extern "C"
{
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() 
{
    println!("Just called a Rust function from C!");
}

fn add_to_counter(inc: u32)
{
    unsafe
    {
        COUNTER += inc;
    }
}

unsafe trait Foo
{
    // method go here
}

unsafe impl Foo for i32
{
    // method implementations go here
}