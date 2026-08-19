/// # References
/// * [Control Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/control-register--uartcr?lang=en)
#[io::register]
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

impl Register {
    pub fn new(
        uart_enable: bool,
        sir_enable: bool,
        sir_low_power_irda_mode: bool,
        loopback_enable: bool,
        transmit_enable: bool,
        receive_enable: bool,
        data_transmit_ready: bool,
        request_to_send: bool,
        out1: bool,
        out2: bool,
        rts_enable: bool,
        cts_enable: bool,
    ) -> Self {
        Self::default()
            .update_uart_enable_bit(uart_enable)
            .update_sir_enable_bit(sir_enable)
            .update_sir_low_power_irda_mode_bit(sir_low_power_irda_mode)
            .update_loopback_enable_bit(loopback_enable)
            .update_transmit_enable_bit(transmit_enable)
            .update_receive_enable_bit(receive_enable)
            .update_data_transmit_ready_bit(data_transmit_ready)
            .update_request_to_send_bit(request_to_send)
            .update_out_bits([out1, out2])
            .update_rts_enable_bit(rts_enable)
            .update_cts_enable_bit(cts_enable)
    }
}
