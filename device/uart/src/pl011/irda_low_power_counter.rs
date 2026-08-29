/// # References
/// * [IrDA Low-Power Counter Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/irda-low-power-counter-register--uartilpr?lang=en)
#[io::register]
pub struct Register {
    low_power_divisor_value: u8,
    __: [u8; 3],
}
