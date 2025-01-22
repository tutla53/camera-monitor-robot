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
        i2c::InterruptHandler as I2cInterruptHandler,
    },
};

assign_resources! {
    control_resources: ControlResources {
        ADC_PERIPHERAL: ADC,
        ADC_HEAD_PIN: PIN_26,
        ADC_BODY_PIN: PIN_27,
        HC_POWER: PIN_2,
        HC_STATE_PIN: PIN_15,
        WAKE_BUTTON: PIN_3,
        UART_TX_PIN: PIN_4,
        UART_PIO_CH: PIO0,
    },
    display_resources: DisplayResources {
        I2C_CH: I2C0,
        SCL_PIN: PIN_1,
        SDA_PIN: PIN_0,
    },
}

bind_interrupts!(pub struct Irqs {
    I2C0_IRQ => I2cInterruptHandler<peripherals::I2C0>;
    PIO0_IRQ_0 => PioInterruptHandler<peripherals::PIO0>;
    USBCTRL_IRQ => UsbInterruptHandler<peripherals::USB>;
    ADC_IRQ_FIFO => AdcInterruptHandler;
});

