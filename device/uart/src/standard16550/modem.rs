/// # References
/// * [Modem Control Register](https://www.lookrs232.com/rs232/mcr.htm)
#[io::register]
pub struct Control {
    force_data_terminal_ready: bool,
    force_request_to_send: bool,
    aux_output: [bool; 2],
    loopback_mode: bool,
    autoflow_control: bool,
    __: [bool; 2],
}

impl Control {
    pub fn new(
        force_data_terminal_ready: bool,
        force_request_to_send: bool,
        out1: bool,
        out2: bool,
        loopback_mode: bool,
        autoflow_control: bool,
    ) -> Self {
        Self::default()
            .update_force_data_terminal_ready_bit(force_data_terminal_ready)
            .update_force_request_to_send_bit(force_request_to_send)
            .update_aux_output_bits([out1, out2])
            .update_loopback_mode_bit(loopback_mode)
            .update_autoflow_control_bit(autoflow_control)
    }
}

/// # References
/// * [Modem Status Register](https://www2.denshi.numazu-ct.ac.jp/staff/FreeBSD/2.2.2R/handbook/handbook118.html)
/// * [Modem Status Register](https://www.gowinsemi.com/upload/database_doc/2072/document/626a18f79b6dc.pdf)
/// * [Modem Status Register](https://docs.amd.com/api/khub/documents/9fEQkSYc6HI76WBKkRyBLg/content)
#[io::register]
pub struct Status {
    delta_clear_to_send: bool,
    delda_data_set_ready: bool,
    trailing_edge_ring_indicator: bool,
    delta_data_carrier_detect: bool,
    clear_to_send: bool,
    data_set_ready: bool,
    ring_indicator: bool,
    data_carrier_detect: bool,
}
