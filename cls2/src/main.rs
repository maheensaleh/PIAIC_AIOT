// fn main(){
//     let r ;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("{}",r);
// }


// in this example borrow checker cannot figure iut whteher the 
// the arguments being passed and returned at that point of program
// are having valiid references or not
// that is why we mention life time here

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// corerectoion with lidetime
// fn main() {
//   { let string1 = String::from("abcd");}
//    let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
//
// this will give error
// fn main() {
//     let result:String;
//     let string1 = String::from("abcd");
//    { let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);}
//     println!("The longest string is {}", result);
// }
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// assignment

// #[derive(Debug)]
// struct Post{
//     author :String,
//     about:String,
//     text:String,

// }

// trait Newnotification{
//     fn alert(&self)->String;
// }

// trait Summarize{
//     fn summary(&self)->String;
// }

// impl Newnotification for Post{
//     fn alert(&self)->String{
//         format!("hey you have got a new notification")
//     }
// }

// impl Summarize for Post{
//     fn summary(&self)->String{
//         format!("Post made by{}\nabout {}\ncontext {}",self.author,self.about,self.text)
//     }
// }
// fn notify<T: Newnotification +Summarize>(mess :T){
//     println!("new message!")
// }
// fn main(){
//     let mypost = Post{
//         author: String::from("sara"),
//         about:String::from("weather"),
//         text:String::from("what a great day!"),
//     };
//     // println!("{}",mypost.alert());
//     // println!("{}",mypost.summary());
//     notify(mypost);
// }

//TRAITS BY SIR SAIF

struct Superman{
    name : String
}

struct Spiderman{
    name : String
}

struct Ironman{
    name : String
}

trait Power{
    fn score(&self)->i32{
        50
    }
}
impl Power for Ironman{
    fn score(&self)->i32{
        100
    }
}
impl Power for Spiderman{}

//generic function single
fn notify<T:Power>(hero : T)->String{  //alternative signature : fn notify(hero : impl Power)
    format!("{}",hero.score())
}

//generic function multiple
// fn notify2<t:Power,u:Power>(hero1:t,hero2:u)->String{
//     format!("{} and {}",hero1.score(),hero2.score())
// }
// fn main(){
//     let hero = Spiderman{
//         name : String::from("peter parker")
//     };
    
//     println!("{}",hero.score());

//     let hero2 = Ironman{
//         name : String::from("Tony Stark")
//     };

//     println!("{}",hero2.score());
//     // println!("{}",notify(hero2));
//     println!("{}",notify2(hero,hero2))


// }
 //////////////////////// closures /////////

// fn main(){
//     let mut x = 1;
//     let mut f = || {x = x+1;println!("x = {}",x)};
//     println!("{}",x);
//     f();
//     f();
//     println!("{}",x);

//     let a = || println!("hey there");
//     a();

//     let mut b = |y| y+1;
//     let m = b(3);
//     println!("{}",m);

//     /// capture value from environmwnt
//     let me = "maheen";
//     let mut name  = || me;
//     let myname = name();
//     println!("{}",myname);
// }

// fn main(){

//     let mut y = |mut a,mut b| {
//         a = a+1;
//         b = b+1;
//         println!("{} {}",a,b)
//     };

//     let mut a= 2;
//     let mut b = 3;
//     y(a,b);
//     // println!("{} {} ",a,b);
// }

// fn hello<T:Fn()>(x:T){
//     x();
// }

// fn main(){
//     let mut a = || println!("bye bye");
//     hello(a);
// }

fn hello1<T:Fn(i32)->i32>(x:T)->i32{
    x(2)
}

fn main(){

    let mut a  = |y| y+1;
    println!("{}",hello1(a));
}