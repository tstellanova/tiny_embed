#![no_main]
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

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {
        let _garbage = test(5000, 0);
    }
}