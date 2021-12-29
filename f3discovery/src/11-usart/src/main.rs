#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();

    let bytes = "\n\rThe quick brown fox jumps over the lazy dog.".bytes();
    let start = mono_timer.now();
    for b in bytes {
        while usart1.isr.read().txe().bit_is_clear() {}
        usart1.tdr.write(|w| w.tdr().bits(u16::from(b)));
    }
    let elapsed = start.elapsed();
    iprintln!(&mut itm.stim[0], "loop took {} ticks ({} us)", elapsed, elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6);
    loop {}
}
