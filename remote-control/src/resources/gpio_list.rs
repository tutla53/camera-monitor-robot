//! Resource Allocation Module
//!
//! This module defines the hardware resources used by various components of the robot.
//! It uses the `assign_resources` macro to allocate specific pins and peripherals to each component.

use {
    assign_resources::assign_resources,
    embassy_rp::{
        bind_interrupts,
        peripherals,
        pio::InterruptHandler as PioInterruptHandler,
        usb::InterruptHandler as UsbInterruptHandler,
        adc::InterruptHandler as AdcInterruptHandler,
    },
};

assign_resources! {
    led_resources: LedFadeResources {
        PIO_CH: PIO0,
        LED_PIN: PIN_25,
    },

    control_resources: ControlResources {
        ADC_PERIPHERAL: ADC,
        ADC_HEAD_PIN: PIN_26,
        ADC_BODY_PIN: PIN_27,
        UART_PIO_CH: PIO1,
        UART_TX_PIN: PIN_4,
    },
}

bind_interrupts!(pub struct Irqs {
    PIO0_IRQ_0 => PioInterruptHandler<peripherals::PIO0>;
    PIO1_IRQ_0 => PioInterruptHandler<peripherals::PIO1>;
    USBCTRL_IRQ => UsbInterruptHandler<peripherals::USB>;
    ADC_IRQ_FIFO => AdcInterruptHandler;
});

