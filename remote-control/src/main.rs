//! Main

#![no_std]
#![no_main]

mod resources;
mod tasks;

use {
    crate::tasks::{
        fade::fade,
        control_task::control_task,
    },
    crate::resources::gpio_list::{
        Irqs,
        AssignedResources, 
        LedFadeResources, 
        ControlResources, 
    },
    embassy_executor::Spawner,
    embassy_rp::{
        config::Config,
        usb::Driver,
        peripherals::USB,
    },
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

    spawner.spawn(logger_task(driver)).unwrap();
    spawner.spawn(fade(r.led_resources)).unwrap();
    spawner.spawn(control_task(r.control_resources)).unwrap();
}
