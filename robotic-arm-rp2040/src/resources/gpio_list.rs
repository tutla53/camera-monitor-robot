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
    },
};

assign_resources! {
    led_resources: LedFadeResources {
        PIO_CH: PIO0,
        LED_PIN: PIN_25,
    },

    servo_pio_resources: ServoPioResources {
        SERVO_PIO_CH: PIO1,
        SERVO_BODY_PIN: PIN_10,
        SERVO_HEAD_PIN: PIN_12,
        UART_RX_PIN: PIN_5,
    },
}

bind_interrupts!(pub struct Irqs {
    PIO0_IRQ_0 => PioInterruptHandler<peripherals::PIO0>;
    PIO1_IRQ_0 => PioInterruptHandler<peripherals::PIO1>;
    USBCTRL_IRQ => UsbInterruptHandler<peripherals::USB>;
});

