//! Main

#![no_std]
#![no_main]

mod resources;
mod tasks;

use {
    crate::tasks::{
        servo_pio::{body_servo_task, head_servo_task},
        uart_task::uart_task,
    },
    crate::resources::gpio_list::{
        Irqs,
        AssignedResources, 
        HeadServoResources, 
        BodyServoResources, 
        UartResources,
    },
    embassy_executor::Spawner,
    embassy_rp::{
        config::Config,
        usb::Driver,
        peripherals::USB,
    },
    defmt::*,
    {defmt_rtt as _, panic_probe as _},
};

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner){
    let p = embassy_rp::init(Config::default());
    let driver = Driver::new(p.USB, Irqs);

    let r = split_resources!(p);

    unwrap!(spawner.spawn(logger_task(driver)));
    unwrap!(spawner.spawn(uart_task(r.uart_resources)));
    unwrap!(spawner.spawn(head_servo_task(r.head_servo_resources)));
    unwrap!(spawner.spawn(body_servo_task(r.body_servo_resources)));
}
