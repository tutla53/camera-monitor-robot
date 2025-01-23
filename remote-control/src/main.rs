//! Main

#![no_std]
#![no_main]

mod resources;
mod tasks;

use {
    crate::tasks::{
        control_task::control_task,
        display::display,
        button::button_task,
    },
    crate::resources::gpio_list::{
        AssignedResources, 
        ControlResources, 
        DisplayResources,
        ButtonResources,
        Irqs,
    },
    embassy_executor::Spawner,
    embassy_rp::{
        peripherals::USB,
        config::Config,
        usb::Driver,
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
    unwrap!(spawner.spawn(button_task(r.button_resources)));
    unwrap!(spawner.spawn(display(r.display_resources)));   
    unwrap!(spawner.spawn(control_task(r.control_resources)));
}
