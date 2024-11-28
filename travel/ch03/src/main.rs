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