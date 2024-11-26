use std::marker::PhantomData;
use std::fmt::Debug;

fn main() {
    let str_container = Container { value: "Thought is free." };
    println!("{}", str_container.value);
    let ambiguous_container: Container<Option<String>> = Container { value: None };
    println!("{}", ambiguous_container.value.unwrap_or("None".to_string()));

    let short_alt_ambiguous_container = Container::<Option<String>>::new(None);
    println!("{}", short_alt_ambiguous_container.value.unwrap_or("None".to_string()));

    let b = Box::new(42); // Allocates an integer on the heap
    println!("b = {}", b);
    println!("Value in box: {}", *b);

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    print_list(&list);

    let shape: Box<dyn Shape> = Box::new(Circle { radius: 5.0 });
    println!("Area: {}", shape.area());

    let my_poodle: Dog<Poodle> = Dog {
        name: "Jeffrey".into(),
        breed: PhantomData,
    };
    println!("my_poodle: {} {}", my_poodle.name, my_poodle.breed_name());

    let rect = Rectangle::new(2, 3);
    let square = Square::new(5);
    println!(
        "rect has width {}, height {}, and area {}",
        rect.get_width(),
        rect.get_height(),
        rect.get_area()
    );
    println!(
        "square has length {} and area {}",
        square.get_length(),
        square.get_area()
    );

    let dog = Dog3;
    let cat = Cat;
    println!("I am a {}", describe_type(&dog));
    println!("I am a {}", describe_type(&cat));

    println!("I am a {}", describe_type2::<Dog>());
    println!("I am a {}", describe_type2::<Cat>());
}

struct Dog3;
struct Cat;

pub trait SelfDescribing {
    fn describe(&self) -> String;
    fn describe2() -> String;
}

impl SelfDescribing for Dog3 {
    fn describe(&self) -> String {
        "happy little dog".into()
    }
    fn describe2() -> String {
        "happy little dog".into()
    }
}
impl SelfDescribing for Cat {
    fn describe(&self) -> String {
        "curious cat".into()
    }
    fn describe2() -> String {
        "curious cat".into()
    }
}

fn describe_type<T: SelfDescribing>(t: &T) -> String {
    t.describe()
}

fn describe_type2<T: SelfDescribing>() -> String {
    T::describe2()
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

struct Square {
    length: i32,
}

impl Square {
    pub fn new(length: i32) -> Self {
        Self { length }
    }

    pub fn get_length(&self) -> i32 {
        self.length
    }
}

pub trait Rectangular {
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn get_area(&self) -> i32;
}

impl Rectangular for Rectangle {
    fn get_width(&self) -> i32 {
        self.width
    }
    fn get_height(&self) -> i32 {
        self.height
    }
    fn get_area(&self) -> i32 {
        self.width * self.height
    }
}

impl Rectangular for Square {
    fn get_width(&self) -> i32 {
        self.length
    }
    fn get_height(&self) -> i32 {
        self.length
    }
    fn get_area(&self) -> i32 {
        self.length * self.length
    }
}

trait DoesItBark {
    fn it_barks(&self) -> bool;
}

struct Dog2;

impl DoesItBark for Dog2 {
    fn it_barks(&self) -> bool {
        true
    }
}

trait MinimalTrait {}

struct Labrador {}
struct Retriever {}
struct Poodle {}
struct Dachshund {}

struct Dog<Breed> {
    name: String,
    breed: PhantomData<Breed>,
}

impl Dog<Labrador> {
    fn breed_name(&self) -> &str { // the compiler can reasonably conclude that the lifetime of the returned &str will match &self.
        "labrador"
    }
}

impl Dog<Retriever> {
    fn breed_name(&self) -> &str {
        "retriever"
    }
}

impl Dog<Poodle> {
    fn breed_name(&self) -> &str {
        "poodle"
    }
}
impl Dog<Dachshund> {
    fn breed_name(&self) -> &str {
        "dachshund"
    }
}

enum NextNode<T> {
    Next(Box<ListNode<T>>),
    End,
}

struct ListNode<T> {
    data: Box<T>,
    next: NextNode<T>,
}

enum Recursive<T> {
    Next(Box<Recursive<T>>),
    Boxed(Box<T>),
    Optional(Option<T>),
}

fn print_list(list: &List) {
    match list {
        List::Cons(value, next) => {
            print!("{} -> ", value);
            print_list(next); // Recursively print the next element
        }
        List::Nil => println!("Nil"), // End of the list
    }
}

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

enum List {
    Cons(i32, Box<List>), // Recursive definition
    Nil,
}

struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

#[derive(Clone)]
struct ListItem<T>
where
    T: Clone + Debug,
{
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}