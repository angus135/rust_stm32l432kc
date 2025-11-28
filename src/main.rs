#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

const RCC_BASE: u32 = 0x40021000;
const RCC_AHB2ENR: *mut u32 = (RCC_BASE + 0x4C) as *mut u32; // AHB2 peripheral clock enable

const GPIOB_BASE: u32 = 0x48000400;
const GPIOB_MODE: *mut u32 = (GPIOB_BASE + 0x00) as *mut u32;
const GPIOB_TYPE: *mut u32 = (GPIOB_BASE + 0x04) as *mut u32;
const GPIOB_SPEED: *mut u32 = (GPIOB_BASE + 0x08) as *mut u32;
const GPIOB_SET_RESET: *mut u32 = (GPIOB_BASE + 0x18) as *mut u32;

const LED_PIN: u32 = 3;

#[entry]
fn main() -> ! {

    unsafe {
        // Enable GPIOB clock (bit 1)
        let r = RCC_AHB2ENR.read_volatile();
        RCC_AHB2ENR.write_volatile(r | (1 << 1));
    }

    unsafe {
        // 2. Configure PB3 as push-pull output, some speed
        // MODER: PB3 -> 01 (output)
        let mut moder = GPIOB_MODE.read_volatile();
        moder &= !(0b11 << (LED_PIN * 2));
        moder |=  0b01 << (LED_PIN * 2);
        GPIOB_MODE.write_volatile(moder);

        // OTYPER: PB3 push-pull (0)
        let mut otyper = GPIOB_TYPE.read_volatile();
        otyper &= !(1 << LED_PIN);
        GPIOB_TYPE.write_volatile(otyper);

        // OSPEEDR: PB3 medium speed (10)
        let mut ospeed = GPIOB_SPEED.read_volatile();
        ospeed &= !(0b11 << (LED_PIN * 2));
        ospeed |=  0b10 << (LED_PIN * 2);
        GPIOB_SPEED.write_volatile(ospeed);
    }


    let set_register_value : u32 = 0x08;
    let reset_register_value : u32 = 0x80000;


    loop {
        unsafe {
            GPIOB_SET_RESET.write_volatile(set_register_value);
        }
        wait();
        unsafe {
            GPIOB_SET_RESET.write_volatile(reset_register_value);
        }
        wait();
    }
}

fn wait() {
    for _ in 1..100000 {
        asm::nop();
    }
}