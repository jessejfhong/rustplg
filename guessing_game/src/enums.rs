enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaske,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn using_enums() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let coin = Coin::Quarter(UsState::Alaske);
    let value = value_in_cents(&coin);
    println!("{0}", value);

    println!("{:?}", plus_one(Some(2)));

    let dice_roll = 9;
    let win = match dice_roll {
        3 => "Shit",
        5 => "Shit again",
        9 => "You win",
        _ => "Still shit",
    };

    println!("{}", win);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn if_let() {
    let config_mac = Some(3u8);
    match config_mac {
        Some(i) => println!("{}", i),
        None => println!("Got None"),
    };

    // a less verbose to do pattern matching
    if let Some(max) = config_mac {
        println!("{}", max);
    }
}

#[allow(clippy::manual_map)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
    // x.map(|i| => i + 1)
}
