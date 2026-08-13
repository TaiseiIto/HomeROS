/// References
/// * [Interrupt Enable Register](https://www.lookrs232.com/rs232/ier.htm)
#[bit::field(u8)]
pub struct Enable {
    received_data_available: bool,
    transmitter_holding_register_empty: bool,
    receiver_line_status: bool,
    modem_status: bool,
    sleep_mode: bool,
    low_power_mode: bool,
    reserved: bool,
    reserved: bool,
}
