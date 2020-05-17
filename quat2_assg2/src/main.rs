/////////////////////    QUESTION 1 //////////////////////


// use std::io;

// fn inp_string()->String{

//     let mut val = String::new();

//     io::stdin().read_line(&mut val)
//     .expect("Failed to read line");

//     val
// }

// fn main(){
//     let nums = "0123456789";
//     let operations = ['+','-','*','/','^'];
//     // let vals = Vec::new();

//     loop{

//         let usr_input = inp_string();
//         let mut sign = String::new();
//         let mut num1 = String::new();
//         let mut num2 = String::new();

//         let mut chk = false;

//         for i in usr_input.chars(){
//             for j in operations.iter(){
//                 if i == *j{
//                     sign.push(i);
//                     break
//                 }
//             }
//         }

//         if &usr_input[0..1]=="0"{
//             println!("bye");
//             break;
//         }

//         for a in usr_input.chars(){  
//             if a.to_string() == sign{
//                 chk =true;
//             }
//             for b in nums.chars(){
//                 if a==b{
//                     if chk{
//                         num2.push_str(&a.to_string());
//                     }
//                     else{
//                     num1.push_str(&a.to_string());
//                     }
//                 }
//             }
//         }



//         let p= (num1.to_string()).parse::<f32>().expect("please provide valid input");
//         let q= (num2.to_string()).parse::<f32>().expect("please provide valid input");
//         // println!("{}",sign);

//         if sign =="+"{
//             println!("{}",p+q);
//         }
//         else if sign =="-"{
//             println!("{}",p-q);
//         }
//         else if sign =="*"{
//             println!("{}",p*q);
//         }
//         else if sign =="/"{
//             println!("{}",p/q);
//         }
//         else if sign =="^"{
//             println!("{}", f32::powf(p,q));
//         }

//     }

// }






////////////////////////////////////   QUESTION 2 ///////////////////////////////////

// use std::io;
// fn main() {
//     let x = || println!("hello world");//make a closure which takes no argument and prints hello world
//     x();
// }
// fn main() {
//     let x = |a : u32|{a+1};  //Make a closure which takes one u32 data type as argument and returns with adding 1 to it.
//     let y = 1;
//     println!("The function returns: {}",x(y)); 
// }
// fn main() {
//     let mut c = 1;
//     let mut x = ||  c = c+1  ;//Make a closure which captures value of variable "c" from environment and change the value of c with adding 1.
//     x();
//     println!("The new value of c is: {}",c); // should print 2
// }
// Write a function which accepts a closure, and in the funciton body, it calls the closure.
//  The closure accepts no argument and returns nothing. What should the closure be about? Be creative!
// fn get_closure<T:Fn()>(x:T){
//     x();
// }
// fn main() {

//     let myclosure = || {
//     let mut nums: Vec<f32> = Vec::new();
//     let count = 5;

//     println!("enter four numbers and find their largest difference!!");

//     for i in 1..count{
//         let mut num = String::new();

//         io::stdin().read_line(&mut num)
//         .expect("Failed to read line");

//         let num: f32 = num.trim().parse()
//         .expect("Please type a number!");

//         nums.push(num);
//     }

//     let mut largest_change :f32= 0.0;
//     for x in &nums{
//         for y in &nums{
//             let mut change = x-y;
//             if change<0.0{
//                 change= (change*change).sqrt();}
//             if change>largest_change{
//                 largest_change=change;
//             }

//         }
//     }
//     println!("largest difference for the four nums is {}",largest_change);
//     };
//     get_closure(myclosure);

// }
// Write a function which expects a closure as an argument and in the funciton body, 
// it calls the closure. The closure expects u32 argument and returns the u32 value. The closure adds 1 to the argument and returns it.

// fn get_closure< T :Fn(u32)->u32>( my_func :T)->u32{

//         my_func(5)
// }
// fn main() {
 
//     let myclosure = |num| num+1;
//     println!("the function result is {}", get_closure(myclosure))
// }


////////////////////////////////// QUESTION 3 ///////////////////////

// struct Student<T>
// where T :  Fn(f32,f32,f32)->f32
// {
//     name : String,
//     result: T
// }


// impl<T> Student<T>
// where T :  Fn(f32,f32,f32)->f32
// {

//     fn new(name : String,cal_result: T)->Student<T>{
//         Student{
//             name : name,
//             result : cal_result
//         }
//     }

// }

// fn main(){

//     //this closure will be passed to the struct field
//     let compute_result = |x:f32,y:f32,z:f32|{
//         ((x+y+z)/300.0)*100.0    
//     };

//     let my_stud = String::from("ABC");
//     let student1 = Student::new(my_stud,compute_result);
//     let my_stud_result = (student1.result)(70.0,70.0,90.0);
//     println!("my student {} has scored {} % !!",student1.name,my_stud_result);
// }


///////////////////////  QUESTION NO 4 //////////////////////


// Q4. Mr. Asim wants to adopt children from Yateem khana. He wants to adopt those children only who
// have the primary education and are at least bilingual. Mr. Allah Bakhsh is the head of Yateem Khana. He
// is helping Mr Asim to make his visit successful.
// He is showing him 2 children at a time by passing those 2 children(struct) in a function named adopt(). If
// both of the children have both of the above mentioned qualities(traits), then Mr. Asim happily brings
// them to their new home sweet home. How can you help Mr. Allah Bakhsh / Mr.Asim?
// use std::io;

// struct Child{

//     name:String,
//     age:u8

// }

// trait primEdu{
//     fn isPrimEdu(&self)->String;
// }

// trait bilingual{
//     fn isbilingual(&self)->String;
// }

// impl primEdu for Child{
//     fn isPrimEdu(&self)->String{
//         format!("{}'s primary education is completed",self.name)
//     }
// }

// impl bilingual for Child{

//     fn isbilingual(&self)->String{
//         format!("{} is bilingual",self.name)
//     }

// }


// fn adopt<T:primEdu +bilingual>(child1: T, child2: T){

//     println!("these children fulfil your requirements:\n{} and {}\n{} and {}",
//     child1.isPrimEdu(),child1.isbilingual(),child2.isPrimEdu(),child2.isbilingual());
// }

// fn main(){

//     let ali = Child{
//         name:String::from("Ali"),
//         age :9
//     };

//     let ayesha = Child{
//         name :String::from("Ayesha"),
//         age:8
//     };

//     adopt(ali,ayesha);
// }


////////////////////// ////////////QUESTION 5 ////////////////

// Rust’s closures are anonymous functions and they have the following properties:

// 1 - we can save them in a variable and then use them
 
// 2 - pass them as arguments to other functions.

// 3 - Create the closure in one place and then call the closure to evaluate it in a different context.

// 4--Closures can capture values from the scope in which they’re defined. 


