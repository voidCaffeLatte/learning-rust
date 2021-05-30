use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main()
{
    let simulated_user_specified_value = 30;
    let simualted_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simualted_random_number);

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn simulated_expensive_calculation(intensity: u32) -> u32
{
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32)
{
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity > 25
    {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity));

        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity));
    } else {
        if random_number == 3
        {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity));
        }
    }
}

struct Cacher<T> where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T>
    {
        Cacher
        {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32
    {
        if self.values.contains_key(&arg)
        {
            self.values[&arg]
        }
        else
        {
            let value = (self.calculation)(arg);
            self.values.insert(arg, value);
            value
        }

        // println!("{}", self.values.keys().map(|k| k.to_string()).collect::<Vec<_>>().join(","));
        // let result = self.values.entry(arg).or_insert((self.calculation)(arg));
        // *result

        // match self.value
        // {
        //     Some(v) => v,
        //     None => 
        //     {
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         v
        //     }
        // }
    }
}
