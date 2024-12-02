#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_stm32::peripherals::ADC1;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::time::khz;
use embassy_stm32::timer::input_capture::{CapturePin, InputCapture};
use embassy_stm32::timer::{self, Channel};
use embassy_stm32::{adc, bind_interrupts, peripherals};
use embassy_stm32::adc::Adc;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    ADC1_2 => adc::InterruptHandler<ADC1>;
    TIM2 => timer::CaptureCompareInterruptHandler<peripherals::TIM2>;
});

#[embassy_executor::task]
async fn blinky(led: peripherals::PC13) {
    let mut led = Output::new(led, Level::High, Speed::VeryHigh);

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(100).await;

        info!("low");
        led.set_low();
        Timer::after_millis(100).await;
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


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    unwrap!(spawner.spawn(blinky(p.PC13)));
    unwrap!(spawner.spawn(blinky_2(p.PC12)));
    unwrap!(spawner.spawn(read_adc(p.ADC1, p.PB1)));

    let ch3 = CapturePin::new_ch3(p.PA2, Pull::None);
    let mut ic = InputCapture::new(p.TIM2, None, None, Some(ch3), None, Irqs, khz(1000), Default::default());

    loop {
        info!("wait for rising edge");
        ic.wait_for_rising_edge(Channel::Ch3).await;

        let capture_value = ic.get_capture_value(Channel::Ch3);
        info!("new capture! {}", capture_value);
    }
}