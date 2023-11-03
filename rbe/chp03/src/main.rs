#![allow(dead_code)]

// defining a struct with multiple fields
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// a unit struct
struct UnitStruct;

// a tuple struct
struct AnohterTupleStruct(u32, String);

struct Point(i32, i32);

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> i32 {
        let Self {
            top_left: Point(x1, y1),
            bottom_right: Point(x2, y2),
        } = self;

        let width = (x1 - x2).abs();
        let length = (y1 - y2).abs();

        width * length
    }
}

fn square(point: &Point, i: i32) -> Rectangle {
    Rectangle {
        top_left: Point(point.0, point.1),
        bottom_right: Point(i, i),
    }
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

type Event = WebEvent; // create a type alias, useful when the name is too long

fn inspect(event: &WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("{} pressed", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at: x={}, y={}", x, y);
        }
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn use_demo() {
    // keywork "use" for getting enum without manual scoping
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Rich;
    let work = Soldier;

    match status {
        Rich => println!("I am rich"),
        Poor => println!("poor guy"),
    };

    match work {
        Civilian => println!("civil"),
        Soldier => println!("soldier"),
    };
}

// C-like enum, starts at 0
enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn c_like_enum() {
    println!("zero is {}", Number::Zero as i32);
    println!("red {:06x}", Color::Red as i32);
}



enum List {
    Cons(u32, Box<List>),
    Nil
}

use crate::List::*;

impl List {
    fn new() -> Self {
        Nil
    }

    fn prepend(self, value: u32) -> Self {
        Cons(value, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Nil => 0,
            Cons(_, ref tail) => 1 + tail.len()
        }
    }

    fn stringify(&self) -> String {

        fn stringify_inner(list: &List) -> String {
            match *list {
                Nil => format!("Nil"),
                Cons(head, ref tail) => format!("{}, {}", head, stringify_inner(tail))
            }
        }
        
        format!("[ {} ]", stringify_inner(self))
    }
}


// global constant
static LANGUAGE: &str = "Rust";
static mut GG: i32 = 4;
const THRESHOLD: i32 = 10;


fn main() {
    let name = "looper".to_owned();
    let age = 12;
    let _peter = Person { name, age };

    let point = Point(1, 3);
    println!("({}, {})", point.0, point.1);

    let rectangle = Rectangle {
        top_left: Point(3, 1),
        bottom_right: point,
    };

    println!("area: {}", rectangle.area());

    let new_point = Point(3, 4);
    let Point(_x, _y) = new_point;

    let event = Event::KeyPress('x');
    inspect(&event);

    c_like_enum();

    let mut list = List::new();

    list = list.prepend(1)
        .prepend(2)
        .prepend(3);

    println!("length: {}", list.len());
    println!("{}", list.stringify());
}
