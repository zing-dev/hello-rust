use std::rc::Rc;
use tokio::task;
use tokio::task::yield_now;

#[allow(dead_code)]
async fn test_spawn() {
    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    // Do some other work
    println!("Do some other work...");

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}

#[allow(dead_code)]
#[allow(unused_must_use)]
async fn test_spawn2() {
    let v = vec![1, 2, 3];

    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });
}

#[allow(dead_code)]
#[allow(unused_must_use)]
async fn test_spawn3() {
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });
}

#[tokio::main]
async fn main() {
    test_spawn3();
}
