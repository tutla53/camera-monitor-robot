/*
Development of  2 d.o.f Robot
	- MCU: STM32F1
	by Tutla
*/

#include <string.h>
#include <FreeRTOS.h>
#include <task.h>
#include <queue.h>
#include <mcuio.h>
#include <miniprintf.h>
#include <semphr.h>

#include <libopencm3/cm3/cortex.h>
#include <libopencm3/stm32/rcc.h>
#include <libopencm3/stm32/rtc.h>
#include <libopencm3/stm32/gpio.h>
#include <libopencm3/cm3/nvic.h>
#include <libopencm3/stm32/adc.h>

#define mainECHO_TASK_PRIORITY (tskIDLE_PRIORITY + 1)
#define USE_USB		0		// Set to 1 for USB
#define UART1		1		// UART 1

/*Global Variable*/
extern void vApplicationStackOverflowHook(xTaskHandle *pxTask,signed portCHAR *pcTaskName);

void vApplicationStackOverflowHook(xTaskHandle *pxTask,signed portCHAR *pcTaskName) {
	(void)pxTask;
	(void)pcTaskName;
	for(;;);
}

/* mutex_lock()/mutex_unlock()
	- prevents competing tasks from printing in the middle of our own line of text
*/
static uint16_t read_adc(uint8_t channel) {

	/*Read ADC*/
	adc_set_sample_time(ADC1,channel,ADC_SMPR_SMP_239DOT5CYC);
	adc_set_regular_sequence(ADC1,1,&channel);
	adc_start_conversion_direct(ADC1);
	while ( !adc_eoc(ADC1) )
		taskYIELD();
	return adc_read_regular(ADC1);
}

static void gpio_setup(void) {
	rcc_clock_setup_in_hse_8mhz_out_72mhz();
	rcc_periph_clock_enable(RCC_GPIOA);
	rcc_periph_clock_enable(RCC_GPIOC);
	gpio_set_mode(GPIOC,GPIO_MODE_OUTPUT_50_MHZ,GPIO_CNF_OUTPUT_PUSHPULL,GPIO13);
	gpio_set_mode(GPIOA,GPIO_MODE_INPUT,GPIO_CNF_INPUT_ANALOG,GPIO0|GPIO1);
	
}

static void adc_setup(void){
	
	/*Setup ADC*/
	rcc_peripheral_enable_clock(&RCC_APB2ENR,RCC_APB2ENR_ADC1EN);
	adc_power_off(ADC1);
	rcc_peripheral_reset(&RCC_APB2RSTR,RCC_APB2RSTR_ADC1RST);
	rcc_peripheral_clear_reset(&RCC_APB2RSTR,RCC_APB2RSTR_ADC1RST);
	rcc_set_adcpre(RCC_CFGR_ADCPRE_PCLK2_DIV6);	// Set. 12MHz, Max. 14MHz
	adc_set_dual_mode(ADC_CR1_DUALMOD_IND);		// Independent mode
	adc_disable_scan_mode(ADC1);
	adc_set_right_aligned(ADC1);
	adc_set_single_conversion_mode(ADC1);
	adc_set_sample_time(ADC1,ADC_CHANNEL_TEMP,ADC_SMPR_SMP_239DOT5CYC);
	adc_set_sample_time(ADC1,ADC_CHANNEL_VREF,ADC_SMPR_SMP_239DOT5CYC);
	adc_enable_temperature_sensor();
	adc_power_on(ADC1);
	adc_reset_calibration(ADC1);
	adc_calibrate_async(ADC1);
	while ( adc_is_calibrating(ADC1) );
}

static void main_task (void *args) {
	/*It will be changed to ISR*/
	(void)args;

	for (;;) {
		
		int head_val = read_adc(0);
		int body_val = read_adc(1);
			
        if (head_val > 3500) {
			uart1_putc('w');
			gpio_clear(GPIOC,GPIO13);
		}
        else if (head_val < 500) {
			uart1_putc('s');
			gpio_clear(GPIOC,GPIO13);
		}

        if (body_val > 3500) {
			uart1_putc('d');
			gpio_clear(GPIOC,GPIO13);
		}
        else if (body_val < 500) {
			uart1_putc('a');
			gpio_clear(GPIOC,GPIO13);
		}
		
		vTaskDelay(pdMS_TO_TICKS(200));
		gpio_set(GPIOC,GPIO13);
	}
}

int main(void) {
	
	/*Hardware Setup*/
	gpio_setup();
	adc_setup();

	gpio_set(GPIOC,GPIO13);
		
	/*Create Task*/
	xTaskCreate(main_task,"main_task",400,NULL,configMAX_PRIORITIES-1,NULL);

	/*Communication Setup*/
	#if USE_USB
		usb_start(1,1);
		std_set_device(mcu_usb);							// Use USB for std I/O
	#else
		rcc_periph_clock_enable(RCC_GPIOA);					// TX=A9, RX=A10, CTS=A11, RTS=A12
		rcc_periph_clock_enable(RCC_USART1);
		
		gpio_set_mode(GPIOA,GPIO_MODE_OUTPUT_50_MHZ,
			GPIO_CNF_OUTPUT_ALTFN_PUSHPULL,GPIO9|GPIO11);
		gpio_set_mode(GPIOA,GPIO_MODE_INPUT,
			GPIO_CNF_INPUT_FLOAT,GPIO10|GPIO12);
		open_uart(UART1, 9600, "8N1", "w", 0, 0);					// UART1
		//std_set_device(mcu_uart1);							// Use UART1 for std I/O
	#endif

	/*Scheduling and start the program*/
	vTaskStartScheduler();
	
	for (;;)
		;
	return 0;
}