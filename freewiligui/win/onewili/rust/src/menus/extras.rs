//! Extended Functions menu - generated from fwMenuExtras. Do not edit.

use crate::transport::Transport;

pub struct Extras<'a> {
    pub(crate) t: &'a mut Transport,
}

impl<'a> Extras<'a> {
    /// MDIO Functions sub-menu.
    pub fn mdio(self) -> super::mdio::Mdio<'a> {
        super::mdio::Mdio { t: self.t }
    }

    /// TI Bootloader sub-menu.
    pub fn ti_bootloader(self) -> super::ti_bootloader::TiBootloader<'a> {
        super::ti_bootloader::TiBootloader { t: self.t }
    }

    /// Wifi Functions sub-menu.
    pub fn wifi(self) -> super::wifi::Wifi<'a> {
        super::wifi::Wifi { t: self.t }
    }

    /// BT Functions sub-menu.
    pub fn bluetooth_le(self) -> super::bluetooth_le::BluetoothLe<'a> {
        super::bluetooth_le::BluetoothLe { t: self.t }
    }

    /// ESP32 Flasher Functions sub-menu.
    pub fn esp32_flasher(self) -> super::esp32_flasher::Esp32Flasher<'a> {
        super::esp32_flasher::Esp32Flasher { t: self.t }
    }

    /// ZoomIO Functions sub-menu.
    pub fn zoom_io(self) -> super::zoom_io::ZoomIo<'a> {
        super::zoom_io::ZoomIo { t: self.t }
    }

    /// Analog In Functions sub-menu.
    pub fn analog_in(self) -> super::analog_in::AnalogIn<'a> {
        super::analog_in::AnalogIn { t: self.t }
    }

    /// Analog Out & Trigger Functions sub-menu.
    pub fn analog_out(self) -> super::analog_out::AnalogOut<'a> {
        super::analog_out::AnalogOut { t: self.t }
    }

    /// WILEye Functions sub-menu.
    pub fn wil_eye(self) -> super::wil_eye::WilEye<'a> {
        super::wil_eye::WilEye { t: self.t }
    }

    /// CANFD Functions sub-menu.
    pub fn canfd(self) -> super::canfd::Canfd<'a> {
        super::canfd::Canfd { t: self.t }
    }

    /// Logic Analyzer Functions sub-menu.
    pub fn logic_analyzer(self) -> super::logic_analyzer::LogicAnalyzer<'a> {
        super::logic_analyzer::LogicAnalyzer { t: self.t }
    }

    /// Logic Player Functions sub-menu.
    pub fn logic_player(self) -> super::logic_player::LogicPlayer<'a> {
        super::logic_player::LogicPlayer { t: self.t }
    }
}
