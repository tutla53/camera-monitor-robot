//! Resource Allocation Module
//!
//! This module defines the hardware resources used by various components of the robot.
//! It uses the `assign_resources` macro to allocate specific pins and peripherals to each component.

use {
    assign_resources::assign_resources,
    embassy_stm32::{
        peripherals,
        bind_interrupts,
        adc::InterruptHandler as AdcInterruptHandler,
        usart::InterruptHandler as UsartInterruptHandler,
    },
};

assign_resources! {
    adc_resources: AdcResources {
        ADC_PERIPHERALS: ADC1,
        ADC_BASE_PIN: PA0,
        ADC_HEAD_PIN: PA1,
    },

    uart_resources: UartResources {
        UART_PERIPHERALS: USART1,
        RX_PIN: PA10,
        RX_DMA: DMA1_CH5,
        TX_PIN: PA9,
        TX_DMA: DMA1_CH4,
    },
}

bind_interrupts!(pub struct Irqs {
    ADC1_2 => AdcInterruptHandler<peripherals::ADC1>;
    USART1 => UsartInterruptHandler<peripherals::USART1>;
});

