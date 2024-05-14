// used as rust linter
#![deny(clippy::all)]

use std::collections::HashMap;

// constants should always have default value
// const AGE: i32 = (4 + 3) * 4;

enum Animals {
    People,
    Dog,
    Cat,
    Rabbit,
}

struct Person {
    name: String,
    age: u8,
}

struct Point(f64, f64, f64);

impl Point {
    // reference is necessary to reuse object
    fn describe(&self) -> String {
        format!("{} {} {}", self.0, self.1, self.1)
    }

    // pass in self to access an instance
    // fn double(&mut self) {
    //     self.0 *= 2.0;
    //     self.1 *= 2.0;
    //     self.2 *= 2.0;
    // }

    // don't pass in self for static methods
    fn zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }

    fn print(&self) {
        println!("{} {} {}", self.0, self.1, self.2);
    }
}

fn main() {
    // // variable names are snake_case
    // let first_name: &str = "Andrew";
    // let last_name: &str = "Boldi";

    // let name: (&str, &str) = (first_name, last_name);

    // println!("{}", AGE);
    // println!("Hello, {} {}!", first_name, last_name);
    // println!("Hello, {} {}!", name.0, name.1);

    let john = Person {
        name: String::from("John"),
        age: 30,
    };
    println!("{} is {} years old", john.name, john.age);

    let person2 = Person { ..john };

    println!("{} is {} years old", person2.name, person2.age);

    // tuples accessed by . notation
    let point = Point(20.0, 20.0, 20.0);
    println!("{}", point.describe());

    let zeroed_point = Point::zero();
    zeroed_point.print();

    let fluffy = Animals::Dog;

    match fluffy {
        Animals::Cat => println!("Meow!"),
        // default keyword
        _ => {
            println!("Not a dog");
        }
    }

    let values: [i32; 3] = [1, 2, 3];
    println!("{}", &values[0]);
    println!("{}", values.len());
    let doubled = values.iter().map(|x| x * 2);

    let mut vec = vec![1, 2, 3];
    vec.push(4);
    vec.pop();
    vec.clear();
    println!("{:?}", vec);

    let mut dict: HashMap<&str, &str> = HashMap::new();

    dict.insert("foo", "bar");

    println!("{:?}", dict);
    if dict.contains_key("name") {
        println!("yes!");
    } else {
        println!("no");
    }

    for (&k, &v) in &dict {
        println!("{} {}", k, v);
    }

    // get entry from hashmap
    let entry = dict.entry("foo");

    dict.insert("name", "John");
    dict.entry("name").or_insert("Jane Doe"); // add only if that value doesn't exist

    for value in vec.iter() {
        println!("{}", value);
    }

    // Optionals are expressed using Option enum. Can have value or none
    let name: Option<&str> = Some("hi");
    match name {
        Some(name) => println!("Hello, {}", name),
        _ => println!("No name"),
    }

    let name: Option<&str> = None;
    // can also do work with unwrap_or_else
    let unwrapped = name.unwrap_or("John Doe");
    println!("name is {}", unwrapped);

    // check if optional has a value with .is_some
}

// Lifetimes

// in this example, you are returning a pointer to a string that is stored on the function's stack.
// Therefore, it disappears when exiting the function and thus the pointer points to nothing.
// & is a borrowed value
// fn get_full_name() -> &str {
//     "John Doe"
// }

// now we make sure it lives on for the lifetime of the application
fn get_full_name() -> &'static str {
    "John Doe"
}

fn get_john_name() -> String {
    "John Doe".to_string()
}

// syntax <'l1, 'l2> essentially sets the variables for the lifetimes used
// the return lifetime must match with the return value in the method itself
fn get_random_name<'l1, 'l2>(a: &'l1 str, b: &'l2 str) -> &'l2 str {
    b
}

// could also make the lifetimes the same
fn get_random_names<'l>(a: &'l str, b: &'l str) -> &'l str {
    b
}

struct Person2 {
    // reference to a string but doesnt' know how long to make it live
    // name: &str,
}

struct Person3<'a> {
    name: &'a str,
}

// Lifetime Rules
// 1. all paramters need a lifetime operator
// 2. single paramater lifetime is AUTOMATICALLY assigned to output lifetime
// 3. if &self or &mut self is in the parameters, lifetime of self is assigned to output

struct Person4<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

// implementations need lifetime specifiers too
impl<'a> Person4<'a> {
    // since we are passing self into the argument, the lifetime of the &str is assumed to be the lifetime of self
    fn first_char_of_first_name(&self) -> &str {
        // string splicing is with [i..j]
        &self.first_name[0..1]
    }
}

// lifetimers are needed to enums
enum Animal2<'a> {
    Dog { name: &'a str },
}

// C was like wild west. you allocate and deallocate. rust doesn't want to doubly deallocate or forget to deallocate.

// Traits (sort of liek itnerfaces)
// shared functionality

// traits make sure structs or enums implement some functionality

trait HasFullName {
    fn full_name(&self) -> String;
}

// wants Person5 to conform to HasFullName
struct Person5 {
    first_name: String,
    last_name: String,
    age: u8,
}

// for keywor
impl HasFullName for Person5 {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

use std::fmt;

// to initialize an object with the string
trait CanInitalizeWithFullName {
    // use capital self for the return type
    fn new(full_name: &str) -> Self;
}

impl CanInitalizeWithFullName for Person5 {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(" ").collect();
        Person5 {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age: 30,
        }
    }
}

impl fmt::Display for Person5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} is {} years old",
            self.first_name, self.last_name, self.age,
        )
    }
}

fn main2() {
    // sort of like defining a constructor by passing self back
    let person: Person5 = Person5::new("John Doe");

    // implement fmt::Display trait
    // works because we implemented fmt::Display for Person5
    println!("{}", person);
}

// traits as parameters

// trait type for parameter
fn print_full_name_and_age(value: &impl HasFullName) {
    println!("{}", value.full_name())
}

// specify trait as generic and pass that in
// use + so that it works for both traits
fn print_details<T: HasFullName + CanRun>(value: &T) {
    println!("{}", value.full_name())
}

// equivalent as above
fn print_details2<T>(value: &T)
where
    T: HasFullName + CanRun,
{
    println!("{}", value.full_name())
}

trait CanRun {
    fn run(&self);
}

impl CanRun for Person5 {
    fn run(&self) {}
}
