#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

use aux11::{entry, usart1};
use aux11::usart1::RegisterBlock;

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n\r"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n\r"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            while self.usart1.isr.read().txe().bit_is_clear() {}
            self.usart1.tdr.write(|w| w.tdr().bits(u16::from(b)));
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, mut _itm) = aux11::init();

    let mut serial = SerialPort { usart1 };

    loop {
        while serial.usart1.isr.read().rxne().bit_is_clear() {}
        let byte = serial.usart1.rdr.read().rdr().bits() as u8 as char;
        uprint!(serial, "{}", byte);
    }
}
