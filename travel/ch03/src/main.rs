use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    what_type_of_integer_is_this(2);
    let some_value: Option<String> = Some("Hello, Rust!".to_string());
    let none_value: Option<String> = None;
    some_or_none(&some_value);

    let my_tuple: (i32, i32, i32) = (10, 20, 30);
    destructure_tuple(&my_tuple);

    let black_cat = Cat {
        name: String::from("Henry"),
        color: CatColor::Black,
    };
    let cheshire_cat = Cat {
        name: String::from("Penelope"),
        color: CatColor::Cheshire,
    };
    match_on_black_cats(&black_cat);
    match_on_black_cats(&cheshire_cat);

    try_to_write_to_file();

    let bark = || println!("Bark!");
    bark();

    let increment = |value| value + 1;
    increment(1);

    let print_and_increment = |value| {
        println!("{value} will be incremented and returned");
        value + 1
    };
    print_and_increment(5);

    let left_value = || 1;
    let right_value = || 2;
    let adder = |left: fn() -> i32,
    right: fn() -> i32| {
        left() + right()
    };
    println!(
        "A {} + {} = {}",
        left_value(),
        right_value(),
        adder(left_value, right_value)
    );

    let consumable = String::from("cookie");
    //let consumer2 = || consumable;
    let consumer = move || consumable;
    consumer();
    //consumer();

    let data = vec![1, 2, 3];
    let closure2 = move || println!("captured {data:?} by value");
    //let closure3 = || println!("captured {data:?} by value");
    closure2();
    closure2();

    let data = Rc::new(String::from("Hello, Rc!"));
    
    let shared1 = Rc::clone(&data); // Increments the reference count
    let shared2 = Rc::clone(&data);

    println!("Count: {}", Rc::strong_count(&data)); // 3 owners: data, shared1, shared2

    println!("{}", shared1); // Access shared data
    println!("{}", shared2);

    let node1 = Rc::new(Node { value: 1, next: None });
    let node2 = Rc::new(Node { value: 2, next: Some(Rc::clone(&node1)) });

    println!("Node2: {:?}, Count: {}", node2, Rc::strong_count(&node1));

    let data = RefCell::new(5);

    // Borrow mutably
    *data.borrow_mut() += 10;

    // Borrow immutably
    println!("Value: {}", *data.borrow()); // Output: 15

    let dinosaurs = LinkedList::new("Tyrannosaurus Rex");
    let last_item = dinosaurs
        .last()
        .expect("couldn't get the last item");
    println!("last_item='{}'", last_item.borrow().data.borrow());

    let mut dinosaurs = LinkedList::new("Tyrannosaurus Rex");
    dinosaurs.append("Triceratops");
    dinosaurs.append("Velociraptor");
    dinosaurs.append("Stegosaurus");
    dinosaurs.append("Spinosaurus");
    dinosaurs
        .into_iter()
        .for_each(
            |ptr| println!("data={}", ptr.borrow().data.borrow())
        );
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

type ItemData<T> = Rc<RefCell<T>>;
type ListItemPtr<T> = Rc<RefCell<ListItem<T>>>;
struct ListItem<T> {
    data: ItemData<T>,
    next: Option<ListItemPtr<T>>,
}

impl<T> ListItem<T> {
    fn new(t: T) -> Self {
        Self {
            data: Rc::new(RefCell::new(t)),
            next: None,
        }
    }
}
struct LinkedList<T> {
    head: ListItemPtr<T>,
    cur_iter: Option<ListItemPtr<T>>,
}

impl<T> LinkedList<T> {
    fn new(t: T) -> Self {
        Self {
            head: Rc::new(RefCell::new(ListItem::new(t))),
            cur_iter: None,
        }
    }
    fn append(&mut self, t: T) {
        self.last()
            .expect("List was empty, but it should never be")
            .as_ref()
            .borrow_mut()
            .next = Some(Rc::new(RefCell::new(ListItem::new(t))));
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = ListItemPtr<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.cur_iter.clone() {
            None => {
                self.cur_iter = Some(self.head.clone());
            }
            Some(ptr) => {
                self.cur_iter = ptr.borrow().next.clone();
            }
        }
        self.cur_iter.clone()
    }
}

enum ErrorTypes {
    IoError(std::io::Error),
    FormatError(std::fmt::Error),
}
struct ErrorWrapper {
    source: ErrorTypes,
    message: String,
}
impl From<std::io::Error> for ErrorWrapper {
    fn from(source: std::io::Error) -> Self {
        Self {
            source: ErrorTypes::IoError(source),
            message: "there was an IO error!".into(),
        }
    }
}

fn write_to_file2() -> Result<(), ErrorWrapper> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::create("filename")?;
    file.write_all(b"File contents")?;
    Ok(())
}
fn try_to_write_to_file2() {
    match write_to_file2() {
        Ok(()) => println!("Write succeeded"),
        Err(err) => {
            println!("Write failed: {}", err.message)
        }
    }
}

fn write_to_file() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("filename")?;
    file.write_all(b"File contents2")?;
    Ok(())
}

fn try_to_write_to_file() {
    match write_to_file() {
        Ok(()) => println!("Write succeeded"),
        Err(err) => println!("Write failed: {}", err.to_string()),
    }
}

enum CatColor {
    Black,
    Red,
    Chocolate,
    Cinnamon,
    Blue,
    Cream,
    Cheshire,
}

struct Cat {
    name: String,
    color: CatColor,
}

fn match_on_black_cats(cat: &Cat) {
    match cat {
        Cat {
            name,
            color: CatColor::Black,
        } => println!("This is a black cat named {name}"),
        Cat { name, color: _ } => println!("{name} is not a black cat"),
    }
}

enum DistinctTypes {
    Name(String),
    Count(i32),
}

fn match_enum_types(enum_types: &DistinctTypes) {
    match enum_types {
        DistinctTypes::Name(name) => println!("name={name}"),
        DistinctTypes::Count(count) => println!("count={count}"),
    }
}

fn destructure_tuple(tuple: &(i32, i32, i32)) {
    match tuple {
        (first, ..) => {
            println!("First tuple element is {first}")
        }
    }
    match tuple {
        (.., last) => {
            println!("Last tuple element is {last}")
        }
    }
    match tuple {
        (_, middle, _) => {
            println!("The middle tuple element is {middle}")
        }
    }
    match tuple {
        (first, middle, last) => {
            println!("The whole tuple is ({first}, {middle}, {last})")
        }
    }
}

fn some_or_none<T: std::fmt::Display>(option: &Option<T>) {
    match option {
        Some(v) => println!("is some! where v={v}"),
        None => println!("is none :("),
    }
}

fn what_type_of_integer_is_this(value: i32) {
    match value {
        1 => println!("The number one number"),
        2 | 3 => println!("This is a two or a three"),
        4..=10 => println!("This is a number between 4 and 10 (inclusive)"),
        _ => println!("Some other kind of number"),
    }
}