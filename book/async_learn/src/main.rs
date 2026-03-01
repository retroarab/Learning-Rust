use std::time::Duration;
use trpl::{Either, Html, StreamExt};
// the async runtime ( in this case Tokio ) will run another async task while this function is
// first waiting for the get await and the .text() await. So to therozie this for memchr
// An async function allows the runtime to run other async tasks while this function is paused on
// an .await.
async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

fn _page_title(url: &str) -> impl Future<Output = (&str, Option<String>)> {
    async move {
        let text = trpl::get(url).await.text().await;
        let title = Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html());
        (url, title)
    }
}

fn main() {
    trpl::block_on(async {
        // let fut1 = async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };
        // //neat remark, it makes complete sense that await moves it, since after
        // //this there's no real 'task' so to say ?
        // //fut1.await;
        //
        // let fut2 = async {
        //     for i in 1..5 {
        //         println!("hi number {i} from the second task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };
        //
        // trpl::join(fut1, fut2).await;

        let (tx, mut rx) = trpl::channel();
        let tx_fut = async {
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
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
        // this is lazy and won't run
        //trpl::join(tx_fut, rx_fut).await;
    });
}
fn main_test1() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        // let url = &args[1];
        // let url2 = &args[1];
        let fut_title1 = page_title(&args[1]);
        let fut_title2 = page_title(&args[2]);
        let (url, maybe_title) = match ::trpl::select(fut_title1, fut_title2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };
        println!("{url} returned first !");
        match maybe_title {
            Some(title) => println!("With the title being {title}"),
            None => println!("Can you beleive it didn't have a title ? "),
        };
        // match _page_title(url).await {
        //     Some(title) => println!("Title for {url} is {title}"),
        //     None => println!("{url} had not title sir ?"),
        // }
    })
    // match page_title(url)await {
    //     Some(title) => println!("Title for {url} is {title}"),
    //     None => println!("{url} had not title sir ?"),
    // }
}
