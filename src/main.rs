use std::fmt::Debug;
use std::hash::Hash;
use std::thread;
use std::time::Duration;
use rand::Rng;
use rand::prelude::thread_rng;
use std::collections::HashMap;

struct Cacher<T, U>
where
    U: Hash + Eq + Copy + Debug,
    T: Fn(U) -> U,
{
    calculation: T,
    value: HashMap<U, U>,
}

impl<T, U> Cacher<T, U> 
where 
    U: Hash + Eq + Copy + Debug,
    T: Fn(U) -> U,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value.get(&arg) {
            Some(v) => v.clone(),
            None => {
               let v = (self.calculation)(arg);
               self.value.insert(arg, v);
               v
            }    
        }
    }
    fn show(self) { 
        for (key, value) in self.value {
            println!("{:?}: {:?}", key, value)
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("calculatins slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity+2));
        println!("Today, do {} biceps!", cached_result.value(intensity+3));
        println!("Next, do {} muscleups!", cached_result.value(intensity+2));
        println!("Today, do {} pullups!", cached_result.value(intensity));
        println!("Today, do {} pullups!", cached_result.value(intensity+3));

    } else {

        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity));
        }
    }
    cached_result.show();
}

fn main() {
    let simulated_intensity = 24;
    let simulated_random_number = thread_rng().gen_range(1..=5);

    generate_workout(simulated_intensity, simulated_random_number);

    let x = vec![1, 2, 3];
    let equal_to_x = |z| z == x; // you can put `move` in the front of the closure and cap the ref
    let u = vec![1, 2, 3];
    assert!(equal_to_x(u))
}
