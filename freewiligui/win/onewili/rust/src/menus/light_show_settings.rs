//! NeoPixel Light Show menu - generated from fwMenuLightShowSettings. Do not edit.
//! Each method packs its args into a Vec<u8> and calls the host
//! import `ow_call(cmd_index, args, args_len, ret, ret_cap)` via
//! crate::transport::call - the firmware assembles/decodes the
//! wire command natively; there is no encoding/framing here.

use crate::transport::OwError;

pub struct LightShowSettings<'a> {
    #[allow(dead_code)]
    pub(crate) t: &'a mut (),
}

impl<'a> LightShowSettings<'a> {
    /// Channel 1. Turn the built-in NeoPixel light show on or off (fixed pins, not selectable). Wire: `h\s\n\a`
    pub fn channel1(&mut self, value: i32) -> Result<(), OwError> {
        let mut a = crate::transport::Args::new();
        a.i32(value);
        let _r = crate::transport::call(219 /* CMD_HARDWARE_SETTINGS_HOME_LIGHT_SHOW_SETTINGS_CHANNEL1 */, &a)?;
        Ok(())
    }

    /// Channel 2. Not currently used by the firmware; has no effect. Wire: `h\s\n\b`
    pub fn channel2(&mut self, value: i32) -> Result<(), OwError> {
        let mut a = crate::transport::Args::new();
        a.i32(value);
        let _r = crate::transport::call(220 /* CMD_HARDWARE_SETTINGS_HOME_LIGHT_SHOW_SETTINGS_CHANNEL2 */, &a)?;
        Ok(())
    }

    /// Channel 3. Not currently used by the firmware; has no effect. Wire: `h\s\n\c`
    pub fn channel3(&mut self, value: i32) -> Result<(), OwError> {
        let mut a = crate::transport::Args::new();
        a.i32(value);
        let _r = crate::transport::call(221 /* CMD_HARDWARE_SETTINGS_HOME_LIGHT_SHOW_SETTINGS_CHANNEL3 */, &a)?;
        Ok(())
    }

    /// Channel 4. Not currently used by the firmware; has no effect. Wire: `h\s\n\f`
    pub fn channel4(&mut self, value: i32) -> Result<(), OwError> {
        let mut a = crate::transport::Args::new();
        a.i32(value);
        let _r = crate::transport::call(222 /* CMD_HARDWARE_SETTINGS_HOME_LIGHT_SHOW_SETTINGS_CHANNEL4 */, &a)?;
        Ok(())
    }

    /// Use WiliWasm. Not currently used by the firmware; has no effect. Wire: `h\s\n\e`
    pub fn use_wili_wasm(&mut self) -> Result<(), OwError> {
        let a = crate::transport::Args::new();
        let _r = crate::transport::call(223 /* CMD_HARDWARE_SETTINGS_HOME_LIGHT_SHOW_SETTINGS_USE_WILI_WASM */, &a)?;
        Ok(())
    }
}
