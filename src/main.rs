use async_std::task;

// creating an async function wich takes 5 sec to respond
async fn negate(n: i32) -> i32 {
    println!("negating {}", n);
    task::sleep(std::time::Duration::from_secs(5)).await;
    println!("finished waiting for {}", n);
    n * -1
}

// calling the async function using std::task executor
async fn f() -> i32 {
    let neg = negate(5);
// This function is similar to [std::thread::spawn], 
// except it spawns an asynchronous task.
    let async_neg = task::spawn(negate(3));

    task::sleep(std::time::Duration::from_secs(1)).await;

    neg.await + async_neg.await

}

fn main() {
    println!("{}",task::block_on(f()));
    let res = task::spawn(async {
        task::sleep(std::time::Duration::from_secs(5)).await;
        println!("Hello from main 1")
    });
// as soon as res2 is finished it will close the current thread
// so we need to block the thread aysnchronuosly until we get response from res variable
    let _res2 = task::spawn(async {
        println!("Hello from main 2")
    });

    task::block_on(res);
}
