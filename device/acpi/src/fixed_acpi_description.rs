use {
    super::Header,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Fixed ACPI Description Table (FADT)](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#fixed-acpi-description-table-fadt)
#[repr(packed)]
pub struct Table {
    header: Header,
    firmware_ctrl: [u8; 4],
    dsdt: [u8; 4],
    reserved0: u8,
    preferred_pm_profile: u8,
    sci_int: [u8; 2],
    smi_cmd: [u8; 4],
    acpi_enable: u8,
    acpi_disable: u8,
    s4bios_req: u8,
    pstate_cnt: u8,
    pm1a_evt_blk: [u8; 4],
    pm1b_evt_blk: [u8; 4],
    pm1a_cnt_blk: [u8; 4],
    pm1b_cnt_blk: [u8; 4],
    pm2_cnt_blk: [u8; 4],
    pm_tmr_blk: [u8; 4],
    gpe0_blk: [u8; 4],
    gpe1_blk: [u8; 4],
    pm1_evt_len: u8,
    pm1_cnt_len: u8,
    pm2_cnt_len: u8,
    pm_tmr_len: u8,
    gpe0_blk_len: u8,
    gpe1_blk_len: u8,
    gpe1_base: u8,
    cst_cnt: u8,
    p_lvl2_lat: [u8; 2],
    p_lvl3_lat: [u8; 2],
    flush_size: [u8; 2],
    flush_stride: [u8; 2],
    duty_offset: u8,
    duty_width: u8,
    day_alrm: u8,
    mon_alrm: u8,
    century: u8,
    iapc_boot_arch: [u8; 2],
    reserved1: u8,
    flags: [u8; 4],
    reset_reg: [u8; 12],
    reset_value: u8,
    arm_boot_arch: [u8; 2],
    fadt_minor_vision: u8,
    x_firmware_ctrl: [u8; 8],
    x_dsdt: [u8; 8],
    x_pm1a_evt_blk: [u8; 12],
    x_pm1b_evt_blk: [u8; 12],
    x_pm1a_cnt_blk: [u8; 12],
    x_pm1b_cnt_blk: [u8; 12],
    x_pm2_cnt_blk: [u8; 12],
    x_pm2_tmr_blk: [u8; 12],
    x_gpe0_blk: [u8; 12],
    x_gpe1_blk: [u8; 12],
    sleep_control_reg: [u8; 12],
    sleep_status_reg: [u8; 12],
    hypervisor_vendor_identity: [u8; 8],
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("firmware_ctrl", &self.firmware_ctrl)
            .field("dsdt", &self.dsdt)
            .field("reserved0", &self.reserved0)
            .field("preferred_pm_profile", &self.preferred_pm_profile)
            .field("sci_int", &self.sci_int)
            .field("smi_cmd", &self.smi_cmd)
            .field("acpi_enable", &self.acpi_enable)
            .field("acpi_disable", &self.acpi_disable)
            .field("s4bios_req", &self.s4bios_req)
            .field("pstate_cnt", &self.pstate_cnt)
            .field("pm1a_evt_blk", &self.pm1a_evt_blk)
            .field("pm1b_evt_blk", &self.pm1b_evt_blk)
            .field("pm1a_cnt_blk", &self.pm1a_cnt_blk)
            .field("pm1b_cnt_blk", &self.pm1b_cnt_blk)
            .field("pm2_cnt_blk", &self.pm2_cnt_blk)
            .field("pm_tmr_blk", &self.pm_tmr_blk)
            .field("gpe0_blk", &self.gpe0_blk)
            .field("gpe1_blk", &self.gpe1_blk)
            .field("pm1_evt_len", &self.pm1_evt_len)
            .field("pm1_cnt_len", &self.pm1_cnt_len)
            .field("pm2_cnt_len", &self.pm2_cnt_len)
            .field("pm_tmr_len", &self.pm_tmr_len)
            .field("gpe0_blk_len", &self.gpe0_blk_len)
            .field("gpe1_blk_len", &self.gpe1_blk_len)
            .field("gpe1_base", &self.gpe1_base)
            .field("cst_cnt", &self.cst_cnt)
            .field("p_lvl2_lat", &self.p_lvl2_lat)
            .field("p_lvl3_lat", &self.p_lvl3_lat)
            .field("flush_size", &self.flush_size)
            .field("flush_stride", &self.flush_stride)
            .field("duty_offset", &self.duty_offset)
            .field("duty_width", &self.duty_width)
            .field("day_alrm", &self.day_alrm)
            .field("mon_alrm", &self.mon_alrm)
            .field("century", &self.century)
            .field("iapc_boot_arch", &self.iapc_boot_arch)
            .field("reserved1", &self.reserved1)
            .field("flags", &self.flags)
            .field("reset_reg", &self.reset_reg)
            .field("reset_value", &self.reset_value)
            .field("arm_boot_arch", &self.arm_boot_arch)
            .field("fadt_minor_vision", &self.fadt_minor_vision)
            .field("x_firmware_ctrl", &self.x_firmware_ctrl)
            .field("x_dsdt", &self.x_dsdt)
            .field("x_pm1a_evt_blk", &self.x_pm1a_evt_blk)
            .field("x_pm1b_evt_blk", &self.x_pm1b_evt_blk)
            .field("x_pm1a_cnt_blk", &self.x_pm1a_cnt_blk)
            .field("x_pm1b_cnt_blk", &self.x_pm1b_cnt_blk)
            .field("x_pm2_cnt_blk", &self.x_pm2_cnt_blk)
            .field("x_pm2_tmr_blk", &self.x_pm2_tmr_blk)
            .field("x_gpe0_blk", &self.x_gpe0_blk)
            .field("x_gpe1_blk", &self.x_gpe1_blk)
            .field("sleep_control_reg", &self.sleep_control_reg)
            .field("sleep_status_reg", &self.sleep_status_reg)
            .field(
                "hypervisor_vendor_identity",
                &self.hypervisor_vendor_identity,
            )
            .finish()
    }
}

impl super::Table for Table {}
