#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    println!("Hello, world!");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 0;

    match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => move_player(dice_roll)
    }
}

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("state {state:?}!");
            25
        },

    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}