mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use num_traits::{Float, ToPrimitive};
use stack::Stack;

// can only be used for f64 arguments
fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

// the function can be used for any data type that implementes the Float trait
// e.g. f32 and f64
fn solve_float<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

// this function can now be used for any data type that implements
// the ToPrimitive trait (e.g. any numeric data type)
fn solve_primitive<T: ToPrimitive>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

// declaring multiple generic types
fn solve_multiple_primitive<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

// a trait defined a set of methods and/or properties
// in the case of methods, either a default implementation can be provided
// or its signature; in that case, its implementation must be defined
// in the corresponden struct/enum
trait Vehicle {
    // only signatures were defined here
    fn start(&self) -> ();

    fn stop(&self) -> ();
}

struct Car {}

impl Car {
    fn new() -> Car {
        Car {}
    }
}

// implementing the Vehicle for the Car struct
impl Vehicle for Car {
    fn start(&self) {
        println!("starting vehicle...");
    }

    fn stop(&self) {
        println!("stopping vehiucle...");
    }
}

// accepts any type that implements Vehicle
fn start_stop<T: Vehicle>(vehicle: &T) {
    vehicle.start();
    vehicle.stop();
}

// accepts any type of container that works with strings (e.g. Basket, Stack)
fn add_string<T: Container<String>>(cont: &mut T, item: String) {
    cont.put(item);
}

fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    // ERROR - Rust doesn't  allow arithmetic operations over two different numeric data types
    // let c = a + b;

    let a_f64 = a as f64;
    // allowed since both are the same data type
    let c = a_f64 + b;

    // num_traits::ToPrimitive is required for using the .to_f64() function
    // which returns an Option<f64> (helps avoiding cases such as value overflow)
    let a_f64_2 = a.to_f64().unwrap();
    let b_f32 = b.to_f32().unwrap();

    println!("{}", solve_float(a_f64_2, b));
    println!("{}", solve_float(a, b_f32));

    // only works with Strings
    let mut first_basket = Basket::new(String::from("A new item"));
    // only works with i32
    let second_basket = Basket::new(10);
    // only works with booleans
    let third_basket = Basket::new(true);

    // // only works with Strings
    let mut s_stack = Stack::new(vec![String::from("Test item")]);
    // only works with i32
    let n_stack = Stack::new(vec![1, 2, 3]);

    add_string(&mut first_basket, String::from("Another item"));
    add_string(&mut s_stack, String::from("Another test item"));
}
