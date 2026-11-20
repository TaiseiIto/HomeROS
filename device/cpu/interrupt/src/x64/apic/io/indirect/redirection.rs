/// # References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.1.3 Redirection Table Entry 0 (RTE0) - Offset 10h
#[derive(Debug)]
pub struct Entry {
    low: Low,
    high: High,
}

impl Entry {
    pub fn new(low: Low, high: High) -> Self {
        Self { low, high }
    }
}

/// # References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.1.3 Redirection Table Entry 0 (RTE0) - Offset 10h
#[io::register]
pub struct Low {
    vector: u8,
    delivery_mode: [bool; 3],
    destination_mode: bool,
    delivery_status: bool,
    polarity: bool,
    remote_irr: bool,
    trigger_mode: bool,
    mask: bool,
    __: [bool; 15],
}

/// # References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.1.3 Redirection Table Entry 0 (RTE0) - Offset 10h
#[io::register]
pub struct High {
    __: u16,
    extended_destination: u8,
    destination_id: u8,
}
