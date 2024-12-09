//! Main

#![no_std]
#![no_main]

mod resources;
mod tasks;

use {
    crate::tasks::{
        control_task::control_task,
    },
    crate::resources::gpio_list::{
        AssignedResources, 
        LedFadeResources, 
        ControlResources, 
    },
    embassy_executor::Spawner,
    embassy_rp::{
        config::Config,
    },
    {defmt_rtt as _, panic_probe as _},
};

#[embassy_executor::main]
async fn main(spawner: Spawner){
    let p = embassy_rp::init(Config::default());

    let r = split_resources!(p);

    spawner.spawn(control_task(r.control_resources)).unwrap();
}
