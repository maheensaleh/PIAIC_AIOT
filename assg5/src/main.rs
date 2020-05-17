fn main() {

    let a : u8 = 12;
    let b : u8 = 13;

    let c = &a as *const u8;
    let d = &b as *const u8;

    unsafe{
        println!(" first address  : {:?} ", c);
        println!(" second address : {:?} ",d);
    }


    let a : u32 = 12;
    let b : u32 = 13;

    let c = &a as *const u32;
    let d = &b as *const u32;

    unsafe{
        println!(" first address  : {:?} ", c);
        println!(" second address : {:?} ",d);
    }
}
