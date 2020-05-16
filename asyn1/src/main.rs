use std::thread;
use std::time::Duration;
use futures::executor::block_on;

fn get_two_sites(){


    let thread_one = thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        println!("threadone");
    });

    let thread_two = thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        println!("threadtwo");
    });

    thread_one.join();
    thread_two.join();


}

fn main(){
    block_on(get_two_sites_async());
}

async fn get_two_sites_async(){

    let thread_one = t1();
    let thread_two = t2();
    futures::join!(thread_one,thread_two);


}

async fn t1(){

    thread::sleep(Duration::from_secs(5));
    println!("1");

}

async fn t2(){

    thread::sleep(Duration::from_secs(5));
    println!("2");

}