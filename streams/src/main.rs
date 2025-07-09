use std::{pin::pin, time::Duration};

use trpl::{ReceiverStream, Stream, StreamExt};
fn main() {
    println!("Hello, world!");
    trpl::run(async {
        // stream_one().await;
        messages().await
    })
}

async fn messages() {
    let  messages = get_messages().timeout(Duration::from_millis(200));
    let intervals = get_intervals().map(|count| format!("Interval: {count}")).throttle(Duration::from_millis(10)).timeout(Duration::from_secs(10));
    let merged = messages.merge(intervals);
    let mut stream = pin!(merged);

    while let Some(val) = stream.next().await {
        match val {
            Ok(val) =>         println!("message: {val}"),
            Err(reason) => println!("we got an error {reason}")

        }
    }
}

fn get_messages() -> impl Stream<Item = String>{
    let (tx,rx) = trpl::channel::<String>();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"];

   trpl::spawn_task(async move {
     for (index,message) in messages.into_iter().enumerate() {
        let time_to_sleep = if index % 2 == 0  { 100 } else {300};
        trpl::sleep(Duration::from_millis(time_to_sleep)).await;
    if let Err(send_err) =      tx.send(format!("message: {message}")) {
        eprintln!("cannot send message {message}, error {send_err}");
        break;
    }
    }
   });

    ReceiverStream::new(rx)
}


fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count =0;

        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count+=1;
            if let Err(err) =          tx.send(count) {
                eprintln!("err occured {err}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}


async fn stream_one() {
    let values = [1,2,3,4,5,6,7,8,9,0];
    let iter = values.iter().map(|n|  n *2);
    let mut stream  = trpl::stream_from_iter(iter);

    let mut filteres = stream.filter(|val| val % 3 ==0);

    while let Some(val) = filteres.next().await {
        println!("the value was {val}")
    }
}

    
