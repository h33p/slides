use core::future::Future;
use core::pin::Pin;
use core::task::{ready, Context, Poll};
use tokio::time::{sleep, Duration, Sleep};

struct Func3 {
    counter: usize,
    sleep_obj: Option<Sleep>,
}

impl Default for Func3 {
    fn default() -> Self {
        Self {
            counter: 0,
            sleep_obj: None,
        }
    }
}

impl Future for Func3 {
    type Output = usize;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        loop {
            if let Some(sleep_obj) = this.sleep_obj.as_mut() {
                let sleep_obj = unsafe { Pin::new_unchecked(sleep_obj) };

                ready!(sleep_obj.poll(cx));

                this.sleep_obj = None;
                this.counter += 1;
                if this.counter >= 10 {
                    break Poll::Ready(3);
                }
            } else {
                println!(">>> f3: {}", this.counter);
                this.sleep_obj = Some(sleep(Duration::from_millis(100)));
            }
        }
    }
}

async fn example_f1() -> usize {
    for i in 0..10 {
        println!("> example_f1: {i}");
        sleep(Duration::from_millis(200)).await;
    }
    1
}

async fn example_f2() -> usize {
    for i in 0..5 {
        println!(">> example_f2: {i}");
        sleep(Duration::from_millis(400)).await;
    }
    2
}

#[tokio::main]
async fn main() {
    let res = tokio::join! {
        example_f2(),
        example_f1(),
        Func3::default(),
    };

    println!("Res: {res:?}");
}
