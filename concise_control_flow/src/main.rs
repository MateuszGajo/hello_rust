#[derive(Debug)]
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
     let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    Some(10);

        let mut count = 0;
        let coin = Coin::Dime;
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

     let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

}
