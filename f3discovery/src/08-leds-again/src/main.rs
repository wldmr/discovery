#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    // Setup GPIO E (enable and set as outputs)
    // GPIO E 8–15 are connected to the LEDs
    rcc.ahbenr.write(|it| it.iopeen().set_bit());
    gpioe.moder.write(|it| it
        .moder8().output()
        .moder9().output()
        .moder10().output()
        .moder11().output()
        .moder12().output()
        .moder13().output()
        .moder14().output()
        .moder15().output()
    );


    // Turn on all the LEDs in the compass
    gpioe.odr.write(|it| it
        .odr8().set_bit()
        .odr9().set_bit()
        .odr10().set_bit()
        .odr11().set_bit()
        .odr12().set_bit()
        .odr13().set_bit()
        .odr14().set_bit()
        .odr15().set_bit()
    );

    aux8::bkpt();

    loop {}
}
