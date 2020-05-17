// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// use aux5::{entry, prelude::*, Delay, Leds};

// #[entry]
// fn main() -> ! {
//     let (mut delay, mut leds): (Delay, Leds) = aux5::init();

//     let half_period = 100_u16;

//     loop {

//         for i in 0..7{
//         leds[i].on();
//         leds[i+1].on();
//         delay.delay_ms(half_period);
//         leds[i].off();
//         }
//         leds[7].off();
//     }
// }


#![deny(unsafe_code)]  // write unsafe rust code
#![no_main]  // telling that their is no main function
#![no_std]  // noneed to bring all std library

use aux5::{entry,prelude::*,Delay,Leds};

#[entry]
fn maheen()->!{
    let (mut delay, mut leds): (Delay,Leds) = aux5::init();
    let delaytime:u16 = 500;
    loop{

        for x in 0..8{
            leds[x].on();
            delay.delay_ms(delaytime);


        }

        for y in 0..8{
            leds[y].off();

            delay.delay_ms(delaytime);


        }            
        





    }

}