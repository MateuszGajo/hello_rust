use std::{pin::Pin, thread::{self, sleep}, time::Duration};

use trpl::{Either, Html};
 fn main() {
    // println!("Hello, world!");
    // let args: Vec<String> = std::env::args().collect();

    // trpl::run(async {
    //     let url_1 = page_title(&args[1]);
    //     let url_2 = page_title(&args[2]);

    // let (url, maybe_title) = match trpl::race(url_1, url_2).await {
    //     Either::Left(left) => left,
    //     Either::Right(right) => right
    // };

    // println!("{url} returnes first");

    // match maybe_title {
    //     Some(title) => println!("title is {title}"),
    //     None => println!("site doesnt contains title")
    // }
    // })

    // spawn_tasks();
    trpl::run(async {
        control_runtime().await;

    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;

    let title =Html::parse(&response_text)
    .select_first("title")
    .map(|title_el| title_el.inner_html());
    (url, title)
}

fn spawn_tasks() {
    trpl::run(async {
        // let fut_1 = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });

        let fut_1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut_2 = async {for i in 1..5 {
            println!("hi number {i} from the second task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };
    // this workd much better, its fair, meaning it checks each future equally often, alterinating between them, nevet lets one race ahead if the other is ready.
        trpl::join(fut_1, fut_2).await;
    });



}

async fn feature_channel() {
    let (tx, mut rx) = trpl::channel::<String>();

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let features: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(tx_fut), Box::pin(rx_fut)];

        trpl::join_all(features).await;
}

async fn control_runtime() {
//         let a = async {
//             println!("'a' started.");
//             slow("a", 30);
//             slow("a", 10);
//             slow("a", 20);
//             trpl::sleep(Duration::from_millis(50)).await;
//             println!("'a' finished.");
//         };

//         let b = async {
//             println!("'b' started.");
//             slow("b", 75);
//             slow("b", 10);
//             slow("b", 15);
//             slow("b", 350);
//             trpl::sleep(Duration::from_millis(50)).await;
//             println!("'b' finished.");
//         };

//         trpl::race(a, b).await;


              let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn timeout1() {
            let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} run for {ms}ms");
}


// async fn page_title(url: &str) -> Option<String> {
//     let response = trpl::get(url).await;
//     let response_text = response.text().await;

//     Html::parse(&response_text)
//     .select_first("title")
//     .map(|title_el| title_el.inner_html())
// }

fn page_title_feature(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        
    let response = trpl::get(url).await;
    let response_text = response.text().await;

    Html::parse(&response_text)
    .select_first("title")
    .map(|title_el| title_el.inner_html())
    }
}