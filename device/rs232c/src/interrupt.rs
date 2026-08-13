/// # References
/// * [Interrupt Enable Register](https://www.lookrs232.com/rs232/ier.htm)
#[bit::field]
struct Enable {
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
struct Identification {
    pending: bool,
    status: [bool; 2],
    timeout: bool,
    reserved: bool,
    fifo_64byte: bool,
    fifo_usable: bool,
    fifo_enabled: bool,
}

enum Status {
    Modem,
    Transmitted,
    Received,
    ReceiverLine,
}

impl From<&Identification> for Status {
    fn from(identification: &Identification) -> Self {
        match identification.status_bit_read() {
            [false, false] => Self::Modem,
            [true, false] => Self::Transmitted,
            [false, true] => Self::Received,
            [true, true] => Self::ReceiverLine,
        }
    }
}
