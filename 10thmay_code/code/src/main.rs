use std::collections::HashMap;
use async_std::task;
// Result<(),Box<dyn std::error::Error+Send + Sync>>
fn main(){
    reqwest::get("https://www.google.com"){

        Ok( mut res)=>{
            match res.text(){
                Ok(text)=>println!("text {}",text),
                Err(_)=>println!("error")
            }
        }
        Err(_)=>{

        }
    }
s

    // task::block_on(async{

    //     let res = reqwest::blocking::get("https://httpbin.org/ip")?
    //     .json::<HashMap<String,String>>()?;

    //     println!("{:#?}",res);

    //     Ok(())
    // })

    


