#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, DelayMs, ToggleableOutputSwitch, OutputSwitch};
use volatile::Volatile;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds) = aux5::init();

    let step_ms: u16 = 200;
    let volatile_step_ms = Volatile::new_read_only(&step_ms);

    // Setup initial configuration
    leds[7].on().ok();
    leds[0].on().ok();
    leds[1].on().ok();

    let mut flips = FLIPS.iter().cycle();

    loop {
        let (l, r) = flips.next().unwrap();
        leds[*l].toggle().ok();
        leds[*r].toggle().ok();
        delay.delay_ms(volatile_step_ms.read());
    }
}

const FLIPS: [(usize, usize); 8] = [
    (7, 2),
    (0, 3),
    (1, 4),
    (2, 5),
    (3, 6),
    (4, 7),
    (5, 0),
    (6, 1),
];
