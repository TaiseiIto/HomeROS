/// # References
/// * [Modem Control Register](https://www.lookrs232.com/rs232/mcr.htm)
#[bit::field]
struct Control {
    force_data_terminal_ready: bool,
    force_request_to_sent: bool,
    aux_output: [bool; 2],
    loopback_mode: bool,
    autoflow_control: bool,
    reserved: [bool; 2],
}
