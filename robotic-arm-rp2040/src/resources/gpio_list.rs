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
        uart::InterruptHandler as UartInterruptHandler,
    },
};

assign_resources! {
    head_servo_resources: HeadServoResources {
        HEAD_SERVO_PIO_CH: PIO0,
        HEAD_SERVO_PIN: PIN_12,
    },

    body_servo_resources: BodyServoResources {
        BODY_SERVO_PIO_CH: PIO1,
        BODY_SERVO_PIN: PIN_10,
    },

    uart_resources: UartResources{
        UART_CH: UART1,
        UART_RX_PIN: PIN_5,
        UART_DMA_CH: DMA_CH1, 
    },
}

bind_interrupts!(pub struct Irqs {
    PIO0_IRQ_0 => PioInterruptHandler<peripherals::PIO0>;
    PIO1_IRQ_0 => PioInterruptHandler<peripherals::PIO1>;
    UART1_IRQ => UartInterruptHandler<peripherals::UART1>;
    USBCTRL_IRQ => UsbInterruptHandler<peripherals::USB>;
});

