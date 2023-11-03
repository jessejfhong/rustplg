// regular struct
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit struct without any fileds
struct AlwaysEqual;

fn using_structs() {
    // creating a struct instance
    let user = User {
        active: true,
        username: String::from("scrat"),
        email: String::from("scrat@gmail.com"),
        sign_in_count: 1,
    };

    // accessing fields
    let _username = user.username;

    // create a new user using existing user' data
    let mut user1 = User {
        username: String::from("Jesse"),
        ..user // must be in the last row
    };

    // modify a mutable user's field
    user1.active = false;
}

fn bulid_user(username: String, email: String) -> User {
    // stand way to create user
    // let _user = User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1
    // };

    // or use syntax shortcut
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn using_tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Color(r, g, b) = black;
    println!("{0} {1} {2}", r, g, b);
    println!("{0} {1} {2}", origin.0, origin.1, origin.2);
}

fn using_unit_struct() {
    let _unit_struct = AlwaysEqual;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated functions, no self in parameters
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // method
    // &self  ==> shorthand for self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// can have multiple impl blocks for a struct
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn using_rectangle() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let _rect = Rectangle::new(2, 3);
    let _square = Rectangle::square(3);
    let _area = rectangle.area();

    println!("{:?}", rectangle);
    println!("{:#?}", rectangle);
    dbg!(&rectangle);
    println!("Area of a rectangle: {0}", area(&rectangle));
}
