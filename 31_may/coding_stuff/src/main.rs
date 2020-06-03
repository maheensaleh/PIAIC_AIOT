#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
extern crate reqwest;
// use std::collections::HashMap;
// use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
// use std::io::Read;

// http web methods: get post put delete
#[get("/")]

fn hello()->String{

    let path = Path::new("api.json");
    let display  = path.display();

    println!("{}",display);

    let mut file = match File::create(path){ //if file not exits then create and proceed , if exits then also procedd

        Ok(file)=>file,
        Err(_) =>panic!("Could not create file")


    };

    match reqwest::get("https://api.openweathermap.org/data/2.5/weather?q=Lahore&Apikey=4970e4f266675063af77ad454f45ebd6&units=metric"){

        Ok(mut response )=>{
            match response.text(){
                Ok(text)=>match file.write_all(text.as_bytes()){
                    Ok(_)=>println!("Data wirte in file successfull"),
                    Err(_)=>println!("The error in writing file is ")}
            
               Err(_)=>panic!("The response is not coming from server")
        }
    }
        Err(_)=>panic!("There is an error in server connection")
    }


    let mut file =  match File::open(&path){
        Ok(file) => file,

        Err(e) => panic!("the file open error")
        };

    
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap(); // unmanageable //no error ddisplay as in expect

        let json = Json::from_str(&buffer).unwrap();

        let result = format!("The temperature of Karachi is : {} ",json.find_path(&["main"]).unwrap());

        "result".to_string()

}


fn main(){

    rocket::ignite().mount("/",routes![hello]).launch();

}
