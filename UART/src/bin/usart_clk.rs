//! Quick Start for STM32 H743
#![no_std]
#![no_main]
use core::fmt::Write as _;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32h7xx_hal::{nb::block, prelude::*};

#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32h7xx_hal::pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();
    let ccdr = rcc.sysclk(120.MHz()).freeze(pwrcfg, &dp.SYSCFG);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let usart3 = dp
        .USART3
        .serial(
            (gpiod.pd8.into_alternate(), gpiod.pd9.into_alternate()),
            115200.bps(),
            ccdr.peripheral.USART3,
            &ccdr.clocks,
        )
        .unwrap();
    let (mut usart3_tx, mut usart3_rx) = usart3.split();

    writeln!(usart3_tx, "Start echo.").unwrap();

    loop {
        //wait until received a world.
        let received = block!(usart3_rx.read()).unwrap();
        //send out the world.
        block!(usart3_tx.write(received)).ok();
    }
}
