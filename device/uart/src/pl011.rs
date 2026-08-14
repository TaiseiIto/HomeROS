mod data;
mod receive_status;

/// # References
/// * [ARM PrimeCell UART (PL011) Technical Reference Manual](https://support.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers?lang=en)
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    data: data::Register,
    receive_status: receive_status::Register,
}
