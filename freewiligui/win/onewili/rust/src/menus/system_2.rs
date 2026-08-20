//! System Functions menu - generated from fwMenuSystem. Do not edit.
//! Each method packs its args into a Vec<u8> and calls the host
//! import `ow_call(cmd_index, args, args_len, ret, ret_cap)` via
//! crate::transport::call - the firmware assembles/decodes the
//! wire command natively; there is no encoding/framing here.

use crate::transport::OwError;

pub struct System2<'a> {
    #[allow(dead_code)]
    pub(crate) t: &'a mut (),
}

impl<'a> System2<'a> {
    /// Stream Battery Info. Enables or disables streaming of battery info to the host.. Wire: `h\v\s\o`
    pub fn enable_battery_stream(&mut self, enable: i32) -> Result<(), OwError> {
        let mut a = crate::transport::Args::new();
        a.i32(enable);
        let _r = crate::transport::call(399 /* CMD_HARDWARE_DISPLAY_FUNCTIONS_SYSTEM_ENABLE_BATTERY_STREAM */, &a)?;
        Ok(())
    }

    /// Read OTP Info. Reads bytes from the fused OTP identity blob (bl_otp_info v3). An unprovisioned device reads all zeros. Read in chunks of 256 bytes or less.. Wire: `h\v\s\b`
    pub fn read_otp_info(&mut self, offset: i32, length: i32) -> Result<Vec<u8>, OwError> {
        let mut a = crate::transport::Args::new();
        a.i32(offset);
        a.i32(length);
        let mut _r = crate::transport::call(398 /* CMD_HARDWARE_DISPLAY_FUNCTIONS_SYSTEM_READ_OTP_INFO */, &a)?;
        Ok(_r.bytes())
    }

    /// Boot UF2. Reboots into the SBL bootloader, which chain-loads the named RAM-app UF2 from the SD card /update directory (card root as fallback). No response is sent on success — the device resets.. Wire: `h\v\s\u`
    pub fn boot_uf2(&mut self, filename: &str) -> Result<(), OwError> {
        let mut a = crate::transport::Args::new();
        a.str(filename);
        let _r = crate::transport::call(400 /* CMD_HARDWARE_DISPLAY_FUNCTIONS_SYSTEM_BOOT_UF2 */, &a)?;
        Ok(())
    }
}
