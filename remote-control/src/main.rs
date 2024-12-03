#![no_std]
#![no_main]

mod resources;
use crate::resources::gpio_list::{
    AssignedResources,
    UartResources,
    AdcResources,
    Irqs,
};

use heapless::String;
use core::fmt::Write;
use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals;
use embassy_stm32::adc::Adc;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn blinky(led: peripherals::PC13) {
    let mut led = Output::new(led, Level::High, Speed::VeryHigh);

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(1000).await;

        info!("low");
        led.set_low();
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::task]
async fn blinky_2(led: peripherals::PC12) {
    let mut led = Output::new(led, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(1000).await;

        info!("low");
        led.set_low();
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::task]
async fn read_adc(adc_pin: peripherals::ADC1, mut pin: peripherals::PB1) {
    
    let mut adc = Adc::new(adc_pin);

    let mut vrefint = adc.enable_vref();
    let vrefint_sample = adc.read(&mut vrefint).await;
    let convert_to_millivolts = |sample| {
        // From http://www.st.com/resource/en/datasheet/CD00161566.pdf
        // 5.3.4 Embedded reference voltage
        const VREFINT_MV: u32 = 1200; // mV

        (u32::from(sample) * VREFINT_MV / u32::from(vrefint_sample)) as u16
    };

    loop {
        let v = adc.read(&mut pin).await;
        info!("--> {} - {} mV", v, convert_to_millivolts(v));
        Timer::after_millis(100).await;
    }
}

#[embassy_executor::task]
async fn uart_task(r: UartResources, u: AdcResources) {

    let mut adc = Adc::new(u.ADC_PERIPHERALS);
    let mut base_pin = u.ADC_BASE_PIN;
    let mut head_pin = u.ADC_HEAD_PIN;

    let mut vrefint = adc.enable_vref();

    let mut config = Config::default();
    config.baudrate = 9600;   
    let mut uart = Uart::new(r.UART_PERIPHERALS, 
                            r.RX_PIN, 
                            r.TX_PIN, 
                            Irqs, 
                            r.TX_DMA, 
                            r.RX_DMA,  
                            config).unwrap(); 
    
    let mut msg: String<8> = String::new();
    let mut value:u8 = 0;

    loop {
        core::write!(&mut msg, "{}\n", value).unwrap();
        uart.write(msg.as_bytes()).await.unwrap();
        uart.flush().await.unwrap();
        
        if(value == 255){value = 0;}
        else {value = value + 1};
        msg.clear();
    }
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let r = split_resources!(p);
    spawner.spawn(uart_task(r.uart_resources, r.adc_resources)).unwrap();
    
}