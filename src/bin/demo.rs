#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use ch32_hal as hal;
use ch32_hal::bind_interrupts;
use core::fmt::Write;
use embassy_executor::Spawner;
use hal::usart::UartTx;

use ch32_i2cdev::i2c_device::{Command, Config, I2cSlave};

bind_interrupts!(struct Irqs {
    I2C1_EV => ch32_i2cdev::i2c_device::EventInterruptHandler<ch32_hal::peripherals::I2C1>;
    I2C1_ER => ch32_i2cdev::i2c_device::ErrorInterruptHandler<ch32_hal::peripherals::I2C1>;
});

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) -> ! {
    //hal::debug::SDIPrint::enable();
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSI;
    let p = hal::init(config);

    let mut uart = UartTx::new_blocking(p.USART1, p.PC0, Default::default()).unwrap();

    let mut config = Config::default();
    config.addr = 0x10;
    config.general_call = false;

    let mut i2c = I2cSlave::new(p.I2C1, config, p.PC1, p.PC2, Irqs);

    writeln!(uart, "Formatted {}\n", 12).unwrap();

    let mut buf = [0; 8];

    let mut val: u16 = 0xd00d;

    loop {
        let res = i2c.listen(&mut buf, &mut uart).await;
        match res {
            Ok(Command::Read) => {
                let val2: [u8; 2] = val.to_le_bytes();
                val += 1;
                match i2c.respond_and_fill(&val2, 0x00, &mut uart).await {
                    Ok(_) => (),
                    Err(e) => writeln!(uart, "Error: {:?}\r", e).unwrap(),
                }
            }
            Ok(other) => writeln!(uart, "I2C: {:?}\r", other).unwrap(),
            Err(e) => writeln!(uart, "Error: {:?}\r", e).unwrap(),
        }
    }
}

use core::panic::PanicInfo;

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
