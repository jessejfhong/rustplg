use std::fmt::{Display, Formatter, UpperHex};

// cannot print because it didn't implement Debug or Display trait.
struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

struct DispalyPrintable(i32);

impl Display for DispalyPrintable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({0}, {1})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0} + {1}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (i, n) in vec.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", i, n)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGB ({:>3}, {:>3}, {:>3})",
            self.red, self.green, self.blue
        )
    }
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "0x{0:0>2X}{1:0>2X}{2:0>2X}",
            self.red, self.green, self.blue
        )
    }
}

pub fn print_plg() {
    // `{}` will be replaced by argument
    println!("{} days", 31);

    // positional arguments
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // formatting output using format character
    println!("Base 10:              {}", 69420); // 69420
    println!("Base 2  (binary)      {:b}", 69420); // 10000111100101100
    println!("Base 8  (octal)       {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal) {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal) {:X}", 69420); // 10F2C

    // padding with 0
    println!("{number:0>5}", number = 1); // 00001
    println!("{number:0<5}", number = 1); // 10000

    // padding with a format specifier
    println!("{number:0>width$}", number = 1, width = 6);

    // use surrounding argument, padding with white space
    let number = 1_f64;
    let width = 5_usize;
    println!("{number:>width$}");

    // use {} to print types that implement Display trait
    // the built-in types already implement it.
    println!("{}", true);

    let pi = 3.141592;
    println!("{:.4}", pi);

    println!("{}", DispalyPrintable(3));

    // use {:?} to print types that implement Debug trait
    println!("{:?}", DebugPrintable(1));

    // pretty print with {:#?}
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    println!(
        "{:#?}",
        Person {
            name: "looper",
            age: 8
        }
    );

    let minmax = MinMax(1, 13);
    println!("{}", minmax);
    println!("{:?}", minmax);

    // to use {:b} to print MinMax, it need to implement fmt::Binary,
    // the following code will compile
    // println!("{:b}", minmax);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("{}", complex);
    println!("{:?}", complex);

    let list = List(vec![5, 2, 3]);
    println!("{}", list);

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        println!("{} {:X}", color, color);
    }
}
