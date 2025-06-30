use std::thread;
#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Blue,
    Red,
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

   

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        } 
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    println!("Hello, world!");

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };
    let x = 1;
    // let add_one_v2 = |x| { x + 1 };


    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!("The user with preferences {:?}, gets {:?}", user_pref1, giveaway1);

    let user_pref2= None;
    let giveaway2 = store.giveaway(user_pref2);

    println!("The user with preferences {:?}, gets {:?}", user_pref2, giveaway2);


    thread()
}


fn capture_references_moving_ownership() {
      let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");
    // let only_borrows2 = ( println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");


    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // we can't use now number, because string was infered first
    // let n = example_closure(5);
}

fn mut_ref() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // println!("before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");
}

fn thread() {
    let list = vec![1,2,3];
    println!("Before defining clousre: {list:?}");

    thread::spawn(move || println!("from thread: {list:?}")).join().unwrap()
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn fnMut() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}


fn fnOnce() {
let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{list:#?}");

      let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}