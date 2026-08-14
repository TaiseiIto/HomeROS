/// # References
/// * [Control Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/control-register--uartcr?lang=en)
#[bit::field]
pub struct Register {
    uart_enable: bool,
    sir_enable: bool,
    sir_low_power_irda_mode: bool,
    __: [bool; 4],
    loopback_enable: bool,
    transmit_enable: bool,
    receive_enable: bool,
    data_transmit_ready: bool,
    request_to_send: bool,
    out: [bool; 2],
    rts_enable: bool,
    cts_enable: bool,
    __: u16,
}
