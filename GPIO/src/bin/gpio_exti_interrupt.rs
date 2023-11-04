#![no_std]
#![no_main]
use core::cell::RefCell;

use cortex_m::{
    interrupt::{free, Mutex},
    peripheral::NVIC,
};
use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use stm32h7xx_hal::{
    gpio::{ExtiPin, Input, Output, PushPull, PB0, PC13},
    interrupt,
    prelude::*,
};

use cortex_m_rt::entry;
static BUTTONBLUE_PIN: Mutex<RefCell<Option<PC13<Input>>>> = Mutex::new(RefCell::new(None));
static LED_GREEN: Mutex<RefCell<Option<PB0<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let ccdr = rcc.sysclk(120.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);

    let led_green: stm32h7xx_hal::gpio::Pin<'B', 0, stm32h7xx_hal::gpio::Output> =
        gpiob.pb0.into_push_pull_output();

    let mut button = gpioc.pc13.into_floating_input();
    button.make_interrupt_source(&mut dp.SYSCFG);
    button.trigger_on_edge(&mut dp.EXTI, stm32h7xx_hal::gpio::Edge::Rising);
    button.enable_interrupt(&mut dp.EXTI);
    unsafe {
        cp.NVIC.set_priority(interrupt::EXTI15_10, 1);
        NVIC::unmask(interrupt::EXTI15_10);
    }
    free(|cs| {
        LED_GREEN.borrow(cs).replace(Some(led_green));
        BUTTONBLUE_PIN.borrow(cs).replace(Some(button));
    });
    loop {}
}
#[interrupt]
fn EXTI15_10() {
    free(|cs| {
        if let Some(button) = BUTTONBLUE_PIN.borrow(cs).borrow_mut().as_mut() {
            if button.check_interrupt() {
                if let Some(led) = LED_GREEN.borrow(cs).borrow_mut().as_mut() {
                    led.toggle();
                    println!("led toggle.")
                }
                button.clear_interrupt_pending_bit();
            }
        }
    });
}
