// #![no_main]
// #![no_std]

// // Include the boot header like this. Otherwise,
// // it may be removed by the linker.
// use imxrt1020evk_fcb as _;

// // Same goes for the panic handler.
// use panic_halt as _;

// // The entry macro adorns your main function.
// use imxrt_rt::entry;

// const LED_OFFSET: u32 = 1 << 5;

// // Register addresses come from the reference manual.
// const IOMUXC_MUX_CTL_PAD_GPIO_AD_B0_05: *mut u32 = 0x401F_80D0 as _;
// const GPIO1_GDIR: *mut u32 = (0x401B_8000 + 0x04) as _;
// const GPIO1_DR_SET: *mut u32 = (0x401B_8000 + 0x84) as _;

// // const GPIO2_GDIR: *mut u32 = (0x401B_C000 + 0x04) as _;
// // const GPIO2_DR_SET: *mut u32 = (0x401B_C000 + 0x84) as _;

// // const GPIO3_GDIR: *mut u32 = (0x401C_0000 + 0x04) as _;
// // const GPIO3_DR_SET: *mut u32 = (0x401C_0000 + 0x84) as _;

// // const GPIO5_GDIR: *mut u32 = (0x400C_0000 + 0x04) as _;
// // const GPIO5_DR_SET: *mut u32 = (0x400C_8000 + 0x84) as _;

// #[entry]
// fn main() -> ! {
//     unsafe {
//         // Configure the pad named "GPIO_11" as a GPIO pin
//         // (as opposed to a UART TX pin, for example).
//         IOMUXC_MUX_CTL_PAD_GPIO_AD_B0_05.write_volatile(5);

//         // Set the GPIO as an output with a RMW operation.
//         let mut gpio1_gdir = GPIO1_GDIR.read_volatile();
//         gpio1_gdir |= LED_OFFSET;
//         GPIO1_GDIR.write_volatile(gpio1_gdir);

//         // Turn on the LED.
//         GPIO1_DR_SET.write_volatile(LED_OFFSET);
//     }
//     loop {
//         unsafe {
//             GPIO1_DR_SET.write_volatile(LED_OFFSET);
//         }
//     }
// }

#![no_main]
#![no_std]

use core::hint::spin_loop;

use imxrt1020evk_fcb as _;
use panic_halt as _;
use imxrt_rt::entry;

const LED: u32 = 5;

const CCM_CCGR1: *mut u32 = 0x400F_C06C as _;
const CCM_CCGR4: *mut u32 = 0x400F_C078 as _;
const IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_05: *mut u32 = 0x401F_80D0 as _;
const IOMUXC_SW_PAD_CTL_PAD_GPIO_AD_B0_05: *mut u32 = 0x401F_8244 as _;
const GPIO1_DR: *mut u32 = 0x401B_8000 as _;
const GPIO1_GDIR: *mut u32 = 0x401B_8004 as _;

#[entry]
fn main() -> ! {
    unsafe {
        // Init LED
        // Enable iomuxc clock
        CCM_CCGR4.write_volatile(CCM_CCGR4.read_volatile() | (0x3 << 2));
        // Disable gpio1 clock
        CCM_CCGR1.write_volatile(CCM_CCGR1.read_volatile() & !(0x3 << 26));
        // Set mux to ALT5 GPIO1_IO05
        IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_05.write_volatile(5);
        // Functional properties
        IOMUXC_SW_PAD_CTL_PAD_GPIO_AD_B0_05.write_volatile(0x10B0);
        // Enable gpio1 clock
        CCM_CCGR1.write_volatile(CCM_CCGR1.read_volatile() | (0x3 << 26));
        // Set GPIO pin 5 to output
        GPIO1_GDIR.write_volatile(GPIO1_GDIR.read_volatile() | (0x1 << LED));
        // Set GPIO pin 5 to high
        GPIO1_DR.write_volatile(GPIO1_DR.read_volatile() | (0x1 << LED));

        loop {
            // Toggle GPIO pin 5
            GPIO1_DR.write_volatile(GPIO1_DR.read_volatile() ^ (0x1 << LED));
            // Delay
            for _ in 0..10000000 {
                spin_loop();
            }
        }
    }
}
