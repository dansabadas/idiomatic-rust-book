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
}

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