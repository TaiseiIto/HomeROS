/// # References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.2.3 EOI Register (EOIR) - Offset FEC00040h
#[io::register]
pub struct Register {
    redirection_entry_clear: u8,
    __: [u8; 3],
}
