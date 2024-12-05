use metrohash::MetroBuildHasher;
use std::collections::HashMap;
use std::{fs::File, io::Read};

fn main() {
    // let s: str = "impossible str";
    print_String(String::from("String"));
    print_str(&String::from("String"));
    print_str("str");
    // print_String("str");

    let array = [0u8; 64];
    let slice: &[u8] = &array;
    let (first_half, second_half) = slice.split_at(32);
    println!(
        "first_half.len()={} second_half.len()={}",
        first_half.len(),
        second_half.len()
    );

    let wordlist = "one,two,three,four";
    for word in wordlist.split(',') {
        println!("word={}", word);
    }

    let mut vec = vec![1, 2, 3];
    let slice = vec.as_slice();
    //vec.resize(10, 0);
    println!("{}", slice[0]);

    let mut map = HashMap::<String, String,  MetroBuildHasher>::default();
    map.insert("hello?".into(), "Hello!".into());
    println!("{:?}", map.get("hello?"));

    let value = 0u8;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0b1u16;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0o2u32;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0x3u64;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 4u128;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));

    println!("Binary (base 2) 0b1111_1111={}", 0b1111_1111);
    println!("Octal (base 8) 0o1111_1111={}", 0o1111_1111);
    println!("Decimal (base 10) 1111_1111={}", 1111_1111);
    println!("Hexadecimal (base 16) 0x1111_1111={}", 0x1111_1111);
    println!("Byte literal b'A'={}", b'A');

    let one = { || 1 }();
    let zero = { || 0 }();
    //println!("{}", one / zero);

    assert_eq!((100i32).checked_div(1i32), Some(100i32));
    assert_eq!((100i32).checked_div(0i32), None);
    let result = (100i32).checked_div(0i32);
    println!("{:?}", result);
    let result = (100i32).checked_div(1i32);
    let rres = result.expect("xpected integer");
    println!("{:?}", rres);

    assert_eq!(0xffu8.wrapping_add(1), 0);
    assert_eq!(0xffffffffu32.wrapping_add(1), 0);
    assert_eq!(0u32.wrapping_sub(1), 0xffffffff);
    assert_eq!(0x80000000u32.wrapping_mul(2), 0);

    let tuple = (1, 2, 3);
    println!("tuple = ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    //println!("tuple = {}", tuple);
    match tuple {
        (one, two, three) => println!("{}, {}, {}", one, two, three),
    }

    let (one, two, three) = tuple;
    println!("{}, {}, {}", one, two, three);

    let tuple_struct = TupleStruct("string value".into());
    println!("{}", tuple_struct.0);

    let mut debuggable_struct = DebuggableStruct::default();
    println!("{:?}", debuggable_struct);
    println!("{:?}", debuggable_struct.clone());
    debuggable_struct.increment_number();
    println!("{:?}", debuggable_struct.incremented_number());

    println!("{:?}", JapaneseDogBreeds::ShibaInu);
    println!("{:?}", JapaneseDogBreeds::ShibaInu as u32);
    println!("{}", StringWrapper::from("Hello, world!").0);

    let mut top_grossing_films = vec!["Avatar", "Avengers: Endgame", "Titanic"];
    let top_grossing_films_mutable_reference =
        &mut top_grossing_films;
    top_grossing_films_mutable_reference.push("Star Wars: The Force Awakens");
    let top_grossing_films_reference = &top_grossing_films;
    println!(
        "Printed using immutable reference: {:#?}",
        top_grossing_films_reference
    );
    let top_grossing_films_moved = top_grossing_films;
    println!("Printed after moving: {:#?}", top_grossing_films_moved);

    // println!("Print using original value: {:#?}", top_grossing_films); //original was moved
    // println!(
    // "Print using mutable reference: {:#?}",
    // top_grossing_films_mutable_reference //invalidated after creating immut ref
    // );
    let mut most_populous_us_cities =
    vec!["New York City", "Los Angeles", "Chicago", "Houston"];
    let most_populous_us_cities_cloned = most_populous_us_cities.clone();
    most_populous_us_cities.push("Phoenix");
    println!("most_populous_us_cities = {:#?}", most_populous_us_cities);
    println!(
        "most_populous_us_cities_cloned = {:#?}",
        most_populous_us_cities_cloned
    );

    let mut list = SinglyLinkedList::new("head");
    list.append("middle");
    list.append("tail");
    let mut item = list.head();
    loop {
        println!("item: {}", item.data());
        if let Some(next_item) = item.next() {
            item = next_item;
        } else {
            break;
        }
    }
}

struct ListItem<T> {
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}

struct SinglyLinkedList<T> {
    head: ListItem<T>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data),
            next: None,
        }
    }

    fn next(&self) -> Option<&Self> {
        if let Some(next) = &self.next {
            Some(next.as_ref())
        } else {
            None
        }
    }

    fn mut_tail(&mut self) -> &mut Self {
        if self.next.is_some() {
            self.next.as_mut().unwrap().mut_tail()
        } else {
            self
        }
    }

    fn data(&self) -> &T {
        self.data.as_ref()
    }
}

impl<T> SinglyLinkedList<T> {
    fn new(data: T) -> Self {
        SinglyLinkedList {
            head: ListItem::new(data),
        }
    }
    fn append(&mut self, data: T) {
        let mut tail = self.head.mut_tail();
        tail.next = Some(Box::new(ListItem::new(data)));
    }
    fn head(&self) -> &ListItem<T> {
        &self.head
    }
}

// #[repr(C)]
// struct GzFileState {
//     have: c_uint,
//     next: *mut c_uchar,
//     pos: i64,
// }

struct Error2(String);

impl From<std::io::Error> for Error2 {
    fn from(other: std::io::Error) -> Self {
        Self(other.to_string())
    }
}

fn read_file2(name: &str) -> Result<String, Error2> {
    let mut f = File::open(name)?;
    let mut output = String::new();
    f.read_to_string(&mut output)?;
    Ok(output)
}

struct StringWrapper(String);

impl From<&str> for StringWrapper {
    fn from(other: &str) -> Self {
        Self(other.into())
    }
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Self {
            message: other.to_string(),
        }
    }
}

fn read_file(name: &str) -> Result<String, Error> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug)]
enum JapaneseDogBreeds {
    AkitaKen,
    HokkaidoInu,
    KaiKen,
    KishuInu,
    ShibaInu,
    ShikokuKen,
}

impl From<u32> for JapaneseDogBreeds {
    fn from(other: u32) -> Self {
        match other {
                other if JapaneseDogBreeds::AkitaKen as u32 == other => {
                    JapaneseDogBreeds::AkitaKen
                }
                other if JapaneseDogBreeds::HokkaidoInu as u32 == other => {
                    JapaneseDogBreeds::HokkaidoInu
                }
                other if JapaneseDogBreeds::KaiKen as u32 == other => {
                    JapaneseDogBreeds::KaiKen
                }
                other if JapaneseDogBreeds::KishuInu as u32 == other => {
                    JapaneseDogBreeds::KishuInu
                }
                other if JapaneseDogBreeds::ShibaInu as u32 == other => {
                    JapaneseDogBreeds::ShibaInu
                }
                other if JapaneseDogBreeds::ShikokuKen as u32 == other => {
                    JapaneseDogBreeds::ShikokuKen
                }
                _ => panic!("Unknown breed!"),
            }
        }
}        
#[derive(Debug, Clone, Default)]
struct DebuggableStruct {
    string: String,
    number: i32,
}

impl DebuggableStruct {
    fn increment_number(&mut self) {
        self.number += 1;
    }
    fn incremented_number(mut self) -> Self {
        self.number += 1;
        self
    }
}

pub struct MixedVisibilityStruct {
    pub name: String,
    pub(crate) value: String,
    //pub(super) number: i32,
}

struct TupleStruct(String);
struct EmptyStruct {}
struct AnotherEmptyStruct;

#[derive(Hash, Eq, PartialEq, Debug)]
struct CompoundKey {
    name: String,
    value: i32,
}

fn print_String(s: String) {
    println!("print_String: {}", s);
}
fn print_str(s: &str) {
    println!("print_str: {}", s);
}