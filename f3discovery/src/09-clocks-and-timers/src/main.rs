#![no_main]
#![no_std]

use aux9::{entry,
           switch_hal::StatefulOutputSwitch,
           switch_hal::ToggleableOutputSwitch,
           tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    tim6.arr.write(|w| w.arr().bits(ms));
    tim6.cr1.write(|w| w.cen().set_bit());
    while !tim6.sr.read().uif().bit_is_set() {}
    tim6.sr.write(|w| w.uif().clear_bit())
}

#[entry]
fn main() -> ! {
    let (ledes, rcc, tim6) = aux9::init();
    let mut leds = ledes.into_array();

    // initialize TIM6
    rcc.apb1enr.write(|w| w
        .tim6en().set_bit()
    );
    tim6.cr1.write(|w| w
        .opm().set_bit()
        .cen().clear_bit()
    );
    tim6.psc.write(|w| w
        .psc().bits(7999)
    );

    let mut patterns = PATTERNS.iter().cycle();
    loop {
        let pattern = patterns.next().expect("Patterns should cycle forever.");
        pattern.apply_to(&mut leds);
        delay(tim6, 100);
    }
}

struct Pattern(u8);

impl Pattern {
    fn is_bit_set(&self, index: usize) -> bool {
        self.0 & (1 << index) != 0
    }

    fn apply_to<T>(&self, leds: &mut [T])
        where T: StatefulOutputSwitch + ToggleableOutputSwitch
    {
        for (index, led) in leds.iter_mut().enumerate() {
            let is_on = led.is_on().unwrap_or_default();
            let should_be_on = self.is_bit_set(index);
            if is_on != should_be_on {
                led.toggle().unwrap_or_default();
            }
        }
    }
}

const PATTERNS: [Pattern; 8] = [
    Pattern(0b10000011),
    Pattern(0b00000111),
    Pattern(0b00001110),
    Pattern(0b00011100),
    Pattern(0b00111000),
    Pattern(0b01110000),
    Pattern(0b11100000),
    Pattern(0b11000001),
];
