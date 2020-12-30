#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

pub fn test(numerator: i32, denominator: i32) -> i32 {
    numerator / denominator
}
