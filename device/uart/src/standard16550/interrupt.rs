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

impl Enable {
    pub fn set(
        received_data_available: bool,
        transmitter_holding_register_empty: bool,
        receiver_line_status: bool,
        modem_status: bool,
        sleep_mode: bool,
        low_power_mode: bool,
    ) -> Self {
        Self::default()
            .update_received_data_available_bit(received_data_available)
            .update_transmitter_holding_register_empty_bit(transmitter_holding_register_empty)
            .update_receiver_line_status_bit(receiver_line_status)
            .update_modem_status_bit(modem_status)
            .update_sleep_mode_bit(sleep_mode)
            .update_low_power_mode_bit(low_power_mode)
    }
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
