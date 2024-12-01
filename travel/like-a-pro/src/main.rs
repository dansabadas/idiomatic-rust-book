use metrohash::MetroBuildHasher;
use std::collections::HashMap;

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
}

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