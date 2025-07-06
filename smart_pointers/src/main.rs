use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop;
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5);

    println!("b = {b}");

    // let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3, Box::new(Nil))))));
    new_func();
    new_func2();

     let m = MyBox::new(String::from("Rust"));
    hello(&m);
    run_drop();
}

fn hello(name: &str) {
    println!("Hello, {name}!");

    // without deref coercion we would need ot write it like this
    //   let m = MyBox::new(String::from("Rust"));
    // hello(&(*m)[..]);
}


fn new_func() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);
}

struct MyBox<T>(T);

impl <T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

impl <T>  MyBox<T> {

    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}


fn new_func2() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);
}


struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping customSmartPointer witht data, {}", self.data);
    }
}

fn run_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

