// you can choose eiryher to unwinding or abort. Unwinding means rust walks back up the stack and celams p the data from each function it encounters. Its quite a work, so u can chose aborting which ends the program without cleaning
// [profile.release]
// panic = 'abort'

use std::fs::File;
use std::fs;
use std::io::{ErrorKind, Read, self};
fn main() {
    panic();
    recoverable_with_result();
}

fn recoverable_with_result() {
    let greeting = File::open("hello.txt");

    // let greeting = match greeting {
    //     Ok(file) => file,
    //     Err(error) => panic!("problem opening the file {error:?}"),
    // };

    //  let greeting = match greeting {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt"){
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file {e:?}"),
    //         },
    //         _ => {
    //             panic!("problem opening the file {error:?}")
    //         }
    //     },
    // };

    //   let greeting = greeting.unwrap_or_else(|error| {
    //     if error.kind() ==  ErrorKind::NotFound {
    //         File::create("helloo.txt").unwrap_or_else(|error| {
    //             panic!("problem creating file")
    //         })
    //     }else {
    //         panic!("problem opening the file: {error:?}")
    //     }
    //   });

    //   let greeting = File::open("hello.txt").unwrap(); // this returns file is exists, or panics
      let greeting = File::open("hello.txt").expect("hello.txt should be included"); // this returns file is exists, or panics
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(E) => return Err(E), 
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let  mut username_file_result = File::open("hello.txt")?;

//     // let mut username_file = match username_file_result {
//     //     Ok(file) => file,
//     //     Err(E) => return Err(E), 
//     // };

//     let mut username = String::new();

//     username_file_result.read_to_string(& mut username);
//     Ok(username)

//     // match username_file.read_to_string(&mut username) {
//     //     Ok(_) => Ok(username),
//     //     Err(e) => Err(e),
//     // }
// }

// even shorter way

//  fn read_username_from_file() -> Result<String, io::Error> { 
//     let mut username = String::new();

//     File::open("aa.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }
// and even more
 fn read_username_from_file() -> Result<String, io::Error> { 
    fs::read_to_string("aa.txt")
}

fn no_return_type() {
    //   can't use ? if return type is icompatible
        // let greeting_file = File::open("hello.txt")?;
}

// fn return_type1() {
//       fs::read_to_string("aa.txt")
// }

fn last_char_of_first_line(text: &str) -> Option<char>{
       text.lines().next()?.chars().last()
}

fn panic() {
    //    panic!("panic");

    let v = vec![1,2,3];
    // v[10];
    match v.get(10) {
        Some(val) => println!("safe"),
        None => println!("no value")
    }
}
