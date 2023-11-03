#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

fn main() {
    // explicitly type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5_i32; // suffix annotation

    let default_float = 3.0; // f64
    let default_integer = 1; // i32

    let mut innferred_type = 12;  // i64, inferred from the next line.
    innferred_type = 2343_i64;

    let mut mutable = 12;
    mutable = 42;

    // mutable is inferred as i32, cannot assign valut of other type
    // mutable = true; 

    // variable can be shadowed
    let mutable = false;

    println!("1 + 2 = {}", 1_u32 + 2);

    println!("1 - 2 = {}", 1_i32 - 2);

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000_u32);

    tuple_plg();
    array_plg();
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

#[derive(Debug)]
struct TupleStruct(i32, i32, bool);

#[derive(Debug)]
struct Matrix(f64, f64, f64, f64);

impl Matrix {
    pub fn transpose(&self) -> Self {
        let Matrix(a, b, c, d) = *self;
        Self(a, c, b, d)
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} {} )\n", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn tuple_plg() {
    // tuple can have many different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64, 'a', true);

    // accessing item using index
    println!("first item: {}", long_tuple.0);
    println!("second item: {}", long_tuple.1);

    // nested tuple
    let nested_tuple = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("nested tuple: {:?}", nested_tuple);

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("reversed pair {:?}", reverse(pair));

    // one item pair
    println!("One element tuple: {:?}", (5u32,)); // need to add a comma
    println!("This is just a integer: {:?}", (5u32));

    // destructuring a tuple
    let tuple = (1, "hello", true);
    let (a, b, c) = tuple;

    println!("tuple struct: {:?}", TupleStruct(1, 2, false));

    let matrix = Matrix(1.2, 1.3, 1.4, 1.5);
    println!("{}", matrix);
    println!("{}", matrix.transpose());
}

fn analyze_slice(slice: &[i32]) {
    println!("first item if the slice: {}", slice[0]);
    println!("the slice has {} items", slice.len());
}

fn array_plg() {
    // array signature [T; length]
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // init ys with 0
    let ys: [i32; 500] = [0; 500];

    // accessing items in array
    println!("{} {}", xs[0], xs[1]);

    // length
    println!("{}", ys.len());

    // bytes
    println!("bytes: {}", std::mem::size_of_val(&xs));

    // borrow the whole array
    analyze_slice(&xs);

    // borrow part of the array
    analyze_slice(&xs[1 .. 3]);

    // empty slice: &[]
    let empty_array: [i32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same as above line

    // array.get return an option
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(val) => println!("{}: {}", i, val),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}