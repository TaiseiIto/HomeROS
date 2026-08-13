/// # References
/// * [Interrupt Enable Register](https://www.lookrs232.com/rs232/ier.htm)
#[bit::field]
pub struct Enable {
    received_data_available: bool,
    transmitter_holding_register_empty: bool,
    receiver_line_status: bool,
    modem_status: bool,
    sleep_mode: bool,
    low_power_mode: bool,
    reserved: [bool; 2],
}

/// # References
/// * [Interrupt Identification Register](https://www.lookrs232.com/rs232/iir.htm)
#[bit::field]
pub struct Identification {
    pending: bool,
    status: [bool; 2],
    timeout: bool,
    reserved: bool,
    fifo_64byte: bool,
    fifo: [bool; 2],
}
