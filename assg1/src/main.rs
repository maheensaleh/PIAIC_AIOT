#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
use rocket::response::content::Html;
extern crate reqwest;
// use std::collections::HashMap;
// use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
// use std::io::Read;

#[get("/<number>")]
fn page2(number: i32) -> String {
    let added = number + 5;
    format!("hello {}",added)
    
}



// #[get("/page2")]
// fn page2() -> Html<&'static str> {
//    Html(r"
//    <h1>Hello form page 2</h1>"
// )

// }


#[get("/")]
fn index() -> Html<&'static str> {
   Html(r"
   <h1>Hello form page 1
   </h1>
   
   "
)

}


fn main(){

    rocket::ignite().mount("/",routes![index,page2]).launch();
    // rocket::ignite().mount("/page2",routes![page2]).launch();

}
