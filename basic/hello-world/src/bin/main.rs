//! Hello World.
//! In this package, we use defmt with RTT to println hello to print "hello world" to the console.
//! I highly recomended you to try learning defmt(https://defmt.ferrous-systems.com/introduction) more.
//! Here I only use the most simply fn from defmt for example.
//! You can also try to set some breakpoints and debug it.
#![no_std]
#![no_main]
use defmt::{error, info, println, warn};
use defmt_rtt as _;
use panic_probe as _;

use stm32h7xx_hal::prelude::*;

use cortex_m_rt::entry;
// Run the following fn, and you will get the output from the console.
#[entry]
fn main() -> ! {
    info!("This is a info.");
    warn!("This is a warn.");
    error!("This is a error.");
    println!("This is a println.");
    loop {}
}
