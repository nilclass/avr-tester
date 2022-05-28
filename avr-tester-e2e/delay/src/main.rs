#![no_std]
#![no_main]

use atmega_hal::clock::MHz16;
use atmega_hal::delay::Delay;
use atmega_hal::{pins, Peripherals};
use avr_hal_generic::prelude::_embedded_hal_blocking_delay_DelayMs;
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut delay = Delay::<MHz16>::new();
    let mut pin = pins.pd0.into_output();

    loop {
        pin.toggle();
        delay.delay_ms(100u8);
    }
}
