//! Quick Start for STM32 H743
#![no_std]
#![no_main]
use core::{cell::RefCell, fmt::Write as _};
use cortex_m::{
    asm::wfi,
    interrupt::{free, Mutex},
};
use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32h7xx_hal::{
    block,
    pac::{self, interrupt, Peripherals, USART3},
    prelude::*,
    serial::{Event, Rx, Tx},
};
static USART3_RX: Mutex<RefCell<Option<Rx<USART3>>>> = Mutex::new(RefCell::new(None));
static USART3_TX: Mutex<RefCell<Option<Tx<USART3>>>> = Mutex::new(RefCell::new(None));
#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();
    let ccdr = rcc.sysclk(120.MHz()).freeze(pwrcfg, &dp.SYSCFG);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let mut usart3 = dp
        .USART3
        .serial(
            (gpiod.pd8.into_alternate(), gpiod.pd9.into_alternate()),
            115200.bps(),
            ccdr.peripheral.USART3,
            &ccdr.clocks,
        )
        .unwrap();
    usart3.listen(Event::Idle);
    let (mut usart3_tx, mut usart3_rx) = usart3.split();
    usart3_tx.listen();
    usart3_rx.listen();

    free(|cs| {
        USART3_RX.borrow(cs).replace(Some(usart3_rx));
        USART3_TX.borrow(cs).replace(Some(usart3_tx));
    });

    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::USART3);
    }
    loop {
        wfi();
    }
}
const BUFFER_MAX: usize = 100;
static mut BUFFER: &mut [u8; BUFFER_MAX] = &mut [0; BUFFER_MAX];
static mut BUFFER_LEN: usize = 0;
#[interrupt]
unsafe fn USART3() {
    free(|cs| {
        if let Some(rx) = USART3_RX.borrow(cs).borrow_mut().as_mut() {
            if rx.is_rxne() {
                let data = block!(rx.read()).unwrap();
                BUFFER[BUFFER_LEN] = data;
                BUFFER_LEN += 1;
                if BUFFER_LEN >= BUFFER_MAX - 1 {
                    BUFFER_LEN = 0;
                };
            } else if rx.is_idle() {
                println!(
                    "Len: {}, Percentage: {}%%",
                    BUFFER_LEN,
                    BUFFER_LEN as f32 / BUFFER_MAX as f32 * 100 as f32
                );
                if let Some(tx) = USART3_TX.borrow(cs).borrow_mut().as_mut() {
                    for item in BUFFER[..BUFFER_LEN].iter_mut() {
                        block!(tx.write(*item)).unwrap();
                    }
                }
                BUFFER_LEN = 0;
                rx.clear_idle();
            }
        }
    })
}
