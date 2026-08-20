//! BT Functions menu - generated from fwMenuBluetoothLE. Do not edit.
//! Each method packs its args into a Vec<u8> and calls the host
//! import `ow_call(cmd_index, args, args_len, ret, ret_cap)` via
//! crate::transport::call - the firmware assembles/decodes the
//! wire command natively; there is no encoding/framing here.

use crate::transport::OwError;

pub struct BluetoothLe<'a> {
    #[allow(dead_code)]
    pub(crate) t: &'a mut (),
}

impl<'a> BluetoothLe<'a> {
    /// Bluetooth Settings sub-menu.
    pub fn ble_settings(self) -> super::ble_settings_2::BleSettings2<'a> {
        super::ble_settings_2::BleSettings2 { t: self.t }
    }

    /// Start BT Advertising. Sets the Host Name for the Bluetooth LE. Wire: `w\b\a`
    pub fn on_start_bt_advertising(&mut self, hostname: &str) -> Result<(), OwError> {
        let mut a = crate::transport::Args::new();
        a.str(hostname);
        let _r = crate::transport::call(344 /* CMD_WIRELESS_BLUETOOTH_LE_ON_START_BT_ADVERTISING */, &a)?;
        Ok(())
    }

    /// Stop BT Advertising. Stops BT Advertising. Wire: `w\b\t`
    pub fn on_stop_bt_advertising(&mut self) -> Result<(), OwError> {
        let a = crate::transport::Args::new();
        let _r = crate::transport::call(345 /* CMD_WIRELESS_BLUETOOTH_LE_ON_STOP_BT_ADVERTISING */, &a)?;
        Ok(())
    }

    /// Scan for BT Devices. Scans for BT devices for a given duration. Wire: `w\b\s`
    pub fn on_scan_bt_devices(&mut self, durationms: i32) -> Result<(), OwError> {
        let mut a = crate::transport::Args::new();
        a.i32(durationms);
        let _r = crate::transport::call(346 /* CMD_WIRELESS_BLUETOOTH_LE_ON_SCAN_BT_DEVICES */, &a)?;
        Ok(())
    }

    /// Toggle Enable Terminal API Mode. Enables BLE to FreeWili Terminal API Mode. Wire: `w\b\e`
    pub fn on_enable_terminal(&mut self) -> Result<(), OwError> {
        let a = crate::transport::Args::new();
        let _r = crate::transport::call(347 /* CMD_WIRELESS_BLUETOOTH_LE_ON_ENABLE_TERMINAL */, &a)?;
        Ok(())
    }
}
