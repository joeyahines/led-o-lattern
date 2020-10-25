#![no_main]
#![no_std]
/// Panic Handler
extern crate panic_semihosting;

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::timer::{Timer, Tim2NoRemap, Event, CountDownTimer};
use cortex_m::interrupt::Mutex;
use core::cell::RefCell;
use stm32f1xx_hal::pac::{Interrupt, interrupt, TIM3, Peripherals};

mod led_value;

use led_value::LEDValue;
use stm32f1xx_hal::delay::Delay;

/// Main function
#[entry]
fn main() -> ! {
    // Grab peripherals
    let p = Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let mut core = cortex_m::Peripherals::take().unwrap();

    // Setup GPIO output pins for PWM
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let c3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let pins = (c1, c2, c3);

    // Setup TIM2 to generate a PWM signal of 1khz
    let (mut pwm1, mut pwm2, mut pwm3)= Timer::tim2(p.TIM2, &clocks, &mut rcc.apb1).pwm::<Tim2NoRemap, _, _, _>(
        pins,
        &mut afio.mapr,
        1.khz(),
    );

    // Set the brightness to dim, in this case this is at max duty
    let max = pwm1.get_max_duty();
    pwm1.set_duty(max);
    pwm2.set_duty(max);
    pwm3.set_duty(max);

    // Enable PWM generation
    pwm1.enable();
    pwm2.enable();
    pwm3.enable();

    // Delay for animation
    let mut delay = Delay::new(core.SYST, clocks);

    // Initial LED state
    let mut led_state = LEDValue::from(0);

    // Update each LED every animation cycle
    loop {
        update_led_state(&mut led_state);
        pwm1.set_duty(get_duty(led_state.red, max));
        pwm2.set_duty(get_duty(led_state.green, max));
        pwm3.set_duty(get_duty(led_state.blue, max));
        delay.delay_ms(5u16);
    }
}

/// Get the duty cycle from the color's brightness
fn get_duty(brightness: u8, max_duty: u16) -> u16 {
    // This functions is a guesstimate based off LED visual brightness
    let brightness = (brightness as i64).pow(2);
    let out = (((max_duty as i64)*(-(brightness << 8) + (1 << 24))) >> 24);

    // Clamp output
    if out > max_duty as i64 {
        max_duty
    }
    else if out < 0 {
        0
    }
    else {
        out as u16
    }
}

/// Update the LED pattern
/// This runs through each of the color channels from dim to bright to back to dim
fn update_led_state(led_value: &mut LEDValue) {
    match led_value.count_state {
        1 => {
            led_value.red -= 1;
            if led_value.red == 0 {
                led_value.count_state = 2
            }
        }
        2 => {
            led_value.blue += 1;
            if led_value.blue == 255 {
                led_value.count_state = 3
            }
        }
        3 => {
            led_value.blue -= 1;
            if led_value.blue == 0 {
                led_value.count_state = 4
            }
        }
        4 => {
            led_value.green += 1;
            if led_value.green == 255 {
                led_value.count_state = 5
            }
        }
        5 => {
            led_value.green -= 1;
            if led_value.green == 0 {
                led_value.count_state = 0
            }
        }
        _ => {
            led_value.red += 1;
            if led_value.red == 255 {
                led_value.count_state = 1
            }
        }
    }
}
