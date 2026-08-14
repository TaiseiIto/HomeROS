/// References
/// * [Interrupt Mask Set/Clear Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/interrupt-mask-set-clear-register--uartimsc?lang=en)
#[bit::field]
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
