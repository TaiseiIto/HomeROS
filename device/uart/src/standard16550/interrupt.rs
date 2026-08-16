/// # References
/// * [Interrupt Enable Register](https://www.lookrs232.com/rs232/ier.htm)
#[io::register]
pub struct Enable {
    received_data_available: bool,
    transmitter_holding_register_empty: bool,
    receiver_line_status: bool,
    modem_status: bool,
    sleep_mode: bool,
    low_power_mode: bool,
    __: [bool; 2],
}

/// # References
/// * [Interrupt Identification Register](https://www.lookrs232.com/rs232/iir.htm)
#[io::register]
pub struct Identification {
    pending: bool,
    status: [bool; 2],
    timeout: bool,
    __: bool,
    fifo_64byte: bool,
    fifo_usable: bool,
    fifo_enabled: bool,
}
