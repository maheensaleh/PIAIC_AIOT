// fn main() {

//     let a = 5;// 101
//     let b = 4;//100
//     println!("{:b},{}",a&b,a&b);
//     println!("{:b},{}",a | b,a | b);
//     println!("{}",!a);

//     //shifting bits

//     //left shift n times = multiply the number by (2 raise to power n)
//     // rigth shift n times = divide the number by (2 raise to power n)

//     println!("{},{:b}",5>>2,5>>2); //right shift
//     println!("{},{:b}",5<<2,5<<2); //left shift
//     // println!("{:b}",3.3); ///iee 7.4 format for conversion


//     let b = "101";
//     let a = "";
//     let mut  p:f32 = (b.len()-1) as f32;
//     for i in b.chars(){
//         let mut n = i.to_string().parse::<f32>();
//         println!("{}",i);
//         println!("{}",p);
//         let mut prod = 2.0_f32.powf(p);
//         p=p-1.0
//     }



// }


/////////////////////////////////////////////////////

// fn main(){
//   // refernecing is for both stack and heap
//   //ownership is for heap only
//     let mut a :i64= 0x7ffd88a1a734;
//     let b = &mut a as *mut i64 ;
//     let c =  &a as *const i64;
//     println!("{:p}",b);
//     // println!("{:p}",a);
//     println!("{:?}",c);


//     //changing a through b i.e changing a vlaue through its addresds, this is called dereferencing
//     unsafe{*b+=2;}
//     println!("{}",a);


//     //system level programing language
//     //one that can be used to write an OS i.e.one that can access our hardware
//     // i.e hardware to software interaction
//     // radox -- an os-- is being written in rust
//     // rust compiler makes sure that there is no invalid memeory access
//     // everything  we write it UNSAFE RUST means taht now we are taking the responsibilty of invald  memeory access 
//     // we will use one unsafe super power23r232323--3-33--3320230r333023r3
//     // unsafe code doesnt mean taht it will always have invalid memeory access,
//     //  it just means that we take the responsibity 

//     // in other languages, if there is memory bug, we have to check teh complete code
//     // in rust if tehre is a memeory bug, we know taht it is somewhere in teh insafe rust block


//     // we cannot have two mut ref at the scope: but we can do so in unsafe rust!, thorugh the raw pointer

// }

// fn main(){
//     // refernecing is for both stack and heap
//     //ownership is for heap only
//       let  a = 0x7ffd88a1a734usize;
//       let b = a as *const i32 ;
//       println!("{:p}",b);
  
//       //changing a through b i.e changing a vlaue through its addresds, this is called dereferencing
//       unsafe{
//         println!("{}",*b);
//     }
  
  
      //system level programing language
      //one that can be used to write an OS i.e.one that can access our hardware
      // i.e hardware to software interaction
      // radox -- an os-- is being written in rust
      // rust compiler makes sure that there is no invalid memeory access
      // everything  we write it UNSAFE RUST means taht now we are taking the responsibilty of invald  memeory access 
      // we will use one unsafe super power23r232323--3-33--3320230r333023r3
      // unsafe code doesnt mean taht it will always have invalid memeory access,
      //  it just means that we take the responsibity 
  
      // in other languages, if there is memory bug, we have to check teh complete code
      // in rust if tehre is a memeory bug, we know taht it is somewhere in teh insafe rust block
  
  
      // we cannot have two mut ref at the scope: but we can do so in unsafe rust!, thorugh the raw pointer
  
  // }
  
fn main(){

    let mut val:f32 = 0.0;
    let b = "1010";
    let a = "";
    let mut  p:f32 = (b.len()-1) as f32;
    let mut val = 0.0;


    for i in b.chars(){

      let bit = (i.to_string()).parse::<f32>().expect("please provide valid input");
      // println!(" bit {}",bit+0.0);


      //find 2 pow index
      let prod = 2.0_f32.powf(p);
      // println!(" prod {}",prod);
      p-=1.0;

      val += bit * prod;
    }
    println!(" val {}",val);


}

// fn main(){

//   let mut dec = 5;
//   let mut  ans_rev = String::new();
//   let mut ans = String::new();

//   while dec>0{
//     let  rem = dec%2;
//     dec  = dec/2;
//     // ans_rev.push_str(&rem.to_string());
//     ans = rem.to_string()+&ans;
//   }
//   // println!("{}",ans_rev);
//   println!("{}",ans);
// }