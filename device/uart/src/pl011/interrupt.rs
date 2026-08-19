pub mod fifo_level;

/// References
/// * [Interrupt Mask Set/Clear Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/interrupt-mask-set-clear-register--uartimsc?lang=en)
/// * [Raw Interrupt Status Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/raw-interrupt-status-register--uartris?lang=en)
/// * [Masked Interrupt Status Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/masked-interrupt-status-register--uartmis?lang=en)
/// * [Interrupt Clear Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/interrupt-clear-register--uarticr?lang=en)
#[io::register]
pub struct Register {
    ri_modem: bool,
    cts_modem: bool,
    dcd_modem: bool,
    dsr_modem: bool,
    receive: bool,
    transmit: bool,
    receive_timeout: bool,
    framing_error: bool,
    parity_error: bool,
    break_erro: bool,
    overrun_error: bool,
    __: [bool; 21],
}

impl Register {
    pub fn new(
        ri_modem: bool,
        cts_modem: bool,
        dcd_modem: bool,
        dsr_modem: bool,
        receive: bool,
        transmit: bool,
        receive_timeout: bool,
        framing_error: bool,
        parity_error: bool,
        break_erro: bool,
        overrun_error: bool,
    ) -> Self {
        Self::default()
            .update_ri_modem_bit(ri_modem)
            .update_cts_modem_bit(cts_modem)
            .update_dcd_modem_bit(dcd_modem)
            .update_dsr_modem_bit(dsr_modem)
            .update_receive_bit(receive)
            .update_transmit_bit(transmit)
            .update_receive_timeout_bit(receive_timeout)
            .update_framing_error_bit(framing_error)
            .update_parity_error_bit(parity_error)
            .update_break_erro_bit(break_erro)
            .update_overrun_error_bit(overrun_error)
    }
}
