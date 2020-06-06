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

#[get("/?<n>")]
fn page2(n: i32) -> String {
    println!("working");
    let added = n + 5;
    format!("hello {}",added)

    
}



// #[get("/page2")]
// fn page2() -> Html<&'static str> {
//    Html(r"
//    <h1>Hello form page 2</h1>"
// )

// }


// #[get("/")]
// fn index() -> Html<&'static str> {
//     Html(r#"
//         <title>Homr</title>
//         <form action="/page2" method="get">
//             <input type="text" name="number" />
//             <button type="submit">Go</button>
//         </form>
//     "#)



use rocket::response::content;

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html(r#"
        <title>Home</title>
        <body>
        <div align="center" style =  " background : #42b0f5;margin-right :20% ; margin-left : 20% ;padding: 4%; margin-top:5vw ">
        <h1>Welcome</h1>
        <h3>Check out the change in background color for each number !</h3>

        <form " action="/" method="get">
        <h2> Enter a number</h2>
        <div><input style = "margin-top:5% ; width : 20%; height :10% ;text-align:center ;font-size:7vw" type="text" name="n" /></div>
        <div><button  style = "margin-top:5% ; font-size: 20" type="submit">Next Page</button></div>
        </form>
        <div>
        <body>
    "#)

}


fn main(){

    rocket::ignite().mount("/",routes![index,page2]).launch();
    // rocket::ignite().mount("/page2",routes![page2]).launch();

}
