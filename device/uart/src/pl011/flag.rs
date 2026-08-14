/// # References
/// * [Flag Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/flag-register--uartfr?lang=en)
#[bit::field]
pub struct Register {
    clear_to_send: bool,
    data_set_ready: bool,
    data_carrier_detect: bool,
    busy: bool,
    receive_fifo_empty: bool,
    transmit_fifo_full: bool,
    receive_fifo_full: bool,
    transmit_fifo_empty: bool,
    ring_indicator: bool,
    reserved: [bool; 23],
}
