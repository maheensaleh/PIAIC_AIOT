// using generics to avoid repetition:

// fn get_largest<T: PartialOrd+Copy>(value :&[T])->T{
//     let mut largest = value[0];
//     for &i in value.iter(){
//         if i > largest{
//             largest = i
//         }
//     }
//     largest
// }

// fn main(){
//     let num = vec!['a','b','c'];
//     println!("{}",get_largest(&num));
// }

//generics in strructs 
// #[derive(Debug)]
// struct Point<A,B> {
//     x: A,
//     y: B,
// }

// fn main() {
//     let work = Point { x: 5.33, y: 4.0 };
//     println!("{:?}",work)
// }

//generics in enums
#[derive(Debug)]

enum Language<A,B>{
    Chinese(A),
    Urdu(B)
}
fn main(){
    let mylang = Language::Urdu("salam");
    println!("{:?}",mylang)
}

//generic in methods

// struct Point<T,U> {
//     x: T,
//     y: U,
// }

// impl<T,U> Point<T,U> {
//     fn get_x(&self) -> &T {
//         &self.x
//     }
// }


// impl Point<f32,f32> { //this block is implementaed when T is f32
//     fn get_xy(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p = Point { x: 1.1, y: 10.3 };

//     println!("p.x = {}", p.get_xy());
// }


//two structs with two generics used together
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c'};

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }


//default implemetation for trait

// #![allow(unused_variables)]
// fn main() {
// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }
// struct NewsArticle{
//     a:String
// }
// impl Summary for NewsArticle {}

// let n1 = NewsArticle{ a:"maheen".to_string()};
// println!("{}",n1.summarize())

// }