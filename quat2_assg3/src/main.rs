// Q1. Write a function which expects a closure with FnOnce() trait and call that closure in the
// function

// fn display_password_once<T:FnOnce()->u32>(clos :T){

//     println!("your password is {}",clos());

//     //password will not be shown again:
//     // println!("your password is {}",clos());  //this will give error
// }

// fn main(){

//     let show_password = ||{90078601} ;
//     display_password_once(show_password);

// }

// ------------------------------------------------------
// Q2. Write a function which expects a closure with FnMut() trait and call that closure in the
// function body.


// fn mut_closure<T: FnMut()->u32>(mut clos: T){
//     println!("{:?}",clos());
//     println!("{:?}",clos());
//     println!("{:?}",clos());

// }

// fn main(){

//     let mut val = 3;
//     let my_clos = ||{val*=2;
//                     val};
//     mut_closure(my_clos);
// }

// ---------------------------------------------------------

// Q3. Create a new thread with spawn and Waiting for All Threads to Finish Using join Handles.

// use std::thread;
// use std::time::Duration;

// fn main(){

//     let myname = "mary poppins".to_string();

//     let handle = thread::spawn(move || {
//         for i in myname.chars(){
//             println!("thread {}",i);
//             thread::sleep(Duration::from_millis(10));
//         }
//     });


//     let yourname = "jim".to_string();
//     for j in yourname.chars(){
//         println!("main   {}",j);
//         thread::sleep(Duration::from_millis(10));

//     }
//     handle.join().unwrap();


// }