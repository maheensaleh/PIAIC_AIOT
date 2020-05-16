use async_std::task;
use surf;

async fn fetch(url : &str)->Result<String,surf:Exception>{
    //this function will fetch the value
    surf.get(url).recv_string.await;
    


}

async fn execute(){
    //this func will print the value
    match fetch("https://www.google.com/").await{

        Ok(a)=>println!("ok is {}",a);
        Err(e)=>println!("error is {}",e);

    }

}


fn main() {
    println!("Hello, world!");
    task::block_on(execute());
}
