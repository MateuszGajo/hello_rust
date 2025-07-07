use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
println!("Hello, world!");
    // threads();
    // channels();
    // channel_multiple_val();
    // channel_multiple_val_and_recivers();
    // mutex();
    mutex_multiple();
}


// fn threads() {
//     let v = vec![1,2,3];
//     let handle =    thread::spawn(move || {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread"); 
//             thread::sleep(Duration::from_millis(1));
//             println!("some me vector {:?}", v);
//         }
//     });

//     handle.join().unwrap();
//     for i in 1..5 {
//         println!("hi number {i} from the main thread");
//         thread::sleep(Duration::from_millis(1));
//     }

// }

fn channels() {
     let (tx, rx) = mpsc::channel::<String>();
     thread::spawn(move || {
        let val = String::from("hi");
    tx.send(val).unwrap();
    // can do that,u dont know when value will be droped in reciver
    // println!("val is {val}")
     });

     let recived = rx.recv().unwrap();
     println!("got: {recived}");
}

fn channel_multiple_val() {
    let (tx,rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recived in rx {
        println!("recived val: {recived}");
    }


}


fn channel_multiple_val_and_recivers() {
    let (tx,rx) = mpsc::channel::<String>();
    let tx1= tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recived in rx {
        println!("recived val: {recived}");
    }


}

fn mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        * num =6;
    }

    println!("m = {m:?}");
}

fn mutex_multiple() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num +=1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("results, {}", *counter.lock().unwrap())
}