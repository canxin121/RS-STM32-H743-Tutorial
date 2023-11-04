//! Here we use push_pull mode of gpio_out for example.
#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::{
    interrupt::{free, Mutex},
    peripheral::NVIC,
};
use defmt::println;
use defmt_rtt as _;

use cortex_m_rt::entry;
use panic_probe as _;
use stm32h7xx_hal::{
    device::{self},
    gpio::{self, ExtiPin},
    interrupt,
    prelude::*,
    pwm,
};
static mut raise: bool = false;
static PWM_USER_LED: Mutex<RefCell<Option<pwm::Pwm<device::TIM1, 0, pwm::ComplementaryDisabled>>>> =
    Mutex::new(RefCell::new(None));
static USER_BTN: Mutex<RefCell<Option<gpio::Pin<'C', 13, gpio::Input>>>> =
    Mutex::new(RefCell::new(None));
#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = stm32h7xx_hal::pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let ccdr = rcc.sysclk(120.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let pe9 = gpioe.pe9.into_alternate();
    let mut pwm: pwm::Pwm<device::TIM1, 0, pwm::ComplementaryDisabled> =
        dp.TIM1
            .pwm(pe9, 10.kHz(), ccdr.peripheral.TIM1, &ccdr.clocks);

    pwm.enable();
    pwm.set_duty(pwm.get_max_duty());

    // // enabe pc13 gpio input exti interrupt
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let mut user_btn: gpio::Pin<'C', 13, gpio::Input> = gpioc.pc13.into_floating_input();
    user_btn.make_interrupt_source(&mut dp.SYSCFG);
    user_btn.trigger_on_edge(&mut dp.EXTI, gpio::Edge::Rising);
    user_btn.enable_interrupt(&mut dp.EXTI);
    unsafe {
        NVIC::unmask(interrupt::EXTI15_10);
    }
    free(|cs| {
        PWM_USER_LED.borrow(cs).replace(Some(pwm));
        USER_BTN.borrow(cs).replace(Some(user_btn));
    });
    loop {}
}

#[interrupt]
fn EXTI15_10() {
    free(|cs| {
        if let Some(btn) = USER_BTN.borrow(cs).borrow_mut().as_mut() {
            if btn.check_interrupt() {
                if let Some(pwm) = PWM_USER_LED.borrow(cs).borrow_mut().as_mut() {
                    let mut duty = pwm.get_duty();
                    unsafe {
                        if raise {
                            duty += 300;
                        } else {
                            duty -= 300;
                        }
                    }
                    if duty >= pwm.get_max_duty() {
                        duty = pwm.get_max_duty();
                        unsafe {
                            raise = false;
                        }
                    } else if duty <= 0 {
                        duty = 300;
                        unsafe {
                            raise = true;
                        }
                    }

                    pwm.set_duty(duty);
                }
                btn.clear_interrupt_pending_bit();
            }
        }
    })
}
