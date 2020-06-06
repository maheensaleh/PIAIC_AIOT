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
use rocket::response::content;


#[get("/?<n>")]
fn page2(n: i32) -> content::Html<String> {

    let added = n + 5;

    let colors = ["#fc5826","#5ef711","#f50f91","#0f35f5","#f5de0f"];
    let mut a = 0;
    if (added<0){
        a = -added;
    }
    else{
        a=added;
    }
    let ind = a%5;
    println!("{}",ind);
    let mut color = colors[0]; 
    if (ind ==0){
        color = colors[0];
    }
    else if (ind==1){
        color = colors[1];
    }
    else if (ind==2){
        color = colors[2];
    }
    else if (ind==3){
        color = colors[3];
    }
    else{
        color = colors[4]
    }
    let res = format!(r#"
    <title>Home</title>
    <body style="background : {}">
    <div align="center" style =  " background : #42b0f5;margin-right :20% ; margin-left : 20% ;padding: 4%; margin-top:5vw ">
    <h1>Welcome</h1>
    <h3>Result : {} </h3>

    <form " action="/" method="get">
    <h2> Enter a number</h2>
    <div><button  style = "margin-top:5% ; font-size: 20" type="submit">Back</button></div>
    </form>
    <div>
    <body>
"#,color,added);
    
content::Html(res)

    
}




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
