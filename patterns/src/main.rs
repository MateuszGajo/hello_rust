fn main() {
    println!("Hello, world!");
}


fn match_pattern() {
    // Match patterns is exhaustive
    // match x {
    //     None => None,
    //     Some(i) => Some(i+1)
    // }

    // _ patterns will matc  anything, its used as last arm
}

fn refutuable_patterns () {
    // Patterns that will match for any possible value passed are irrefutable, e.g x in statement let x =5;
    // Psatterns that can fail to match for possible value are refutuable, an example would be Some(x) in expression if let Some(x) = a_value
}

fn multiple_matching() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything")
    }

    match x {
        1..5 => println!("matching from 1 to 5"),
        _ => println!("anything")
    }
}