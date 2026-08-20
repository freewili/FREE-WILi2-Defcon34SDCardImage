//! TI Bootloader menu - generated from fwMenuTIBootloader. Do not edit.

use crate::encoding;
use crate::transport::{OwError, Transport};

pub struct TiBootloader<'a> {
    pub(crate) t: &'a mut Transport,
}

impl<'a> TiBootloader<'a> {
    /// Enter Bootloader. Makes the TI device enter the bootloader. Wire: `e\t\o`
    pub fn enter_bootloader(&mut self) -> Result<(), OwError> {
        let cmd = String::from("e\\t\\o");
        self.t.call(&cmd)?;
        Ok(())
    }

    /// Ping Bootloader. Tests communication with the bootloader. Wire: `e\t\c`
    pub fn ping_bootloader(&mut self) -> Result<(), OwError> {
        let cmd = String::from("e\\t\\c");
        self.t.call(&cmd)?;
        Ok(())
    }

    /// Erase All Flash. Performs a bulk erase of the firmware. Wire: `e\t\r`
    pub fn erase_all_flash(&mut self) -> Result<(), OwError> {
        let cmd = String::from("e\\t\\r");
        self.t.call(&cmd)?;
        Ok(())
    }

    /// Read CRC32. Calculates the CRC across the specificed memory. Wire: `e\t\f`
    pub fn read_crc32(&mut self, start_address: u32, memory_size: i32) -> Result<Vec<u8>, OwError> {
        let mut cmd = String::from("e\\t\\f");
        encoding::push_hex(&mut cmd, start_address as u64, 1);
        encoding::push_int(&mut cmd, memory_size as i64);
        let resp = self.t.call(&cmd)?;
        let mut toks = resp.split_whitespace();
        let crc_data = encoding::rest_bytes(&mut toks)?;
        Ok(crc_data)
    }

    /// Read Memory 32. Reads 32 bytes of memory at the specificed address. Wire: `e\t\u`
    pub fn read_memory32(&mut self, start_address: u32) -> Result<Vec<u8>, OwError> {
        let mut cmd = String::from("e\\t\\u");
        encoding::push_hex(&mut cmd, start_address as u64, 1);
        let resp = self.t.call(&cmd)?;
        let mut toks = resp.split_whitespace();
        let memory_read = encoding::rest_bytes(&mut toks)?;
        Ok(memory_read)
    }

    /// Write Memory 32. Writes a 32 bit word to device SRAM. Wire: `e\t\p`
    pub fn write_memory32(&mut self, start_address: u32, mem_value: u32) -> Result<(), OwError> {
        let mut cmd = String::from("e\\t\\p");
        encoding::push_hex(&mut cmd, start_address as u64, 1);
        encoding::push_hex(&mut cmd, mem_value as u64, 1);
        self.t.call(&cmd)?;
        Ok(())
    }

    /// Read Bootloader Status. Reads status of last bootloader operation. Wire: `e\t\b`
    pub fn read_boot_loader_status(&mut self) -> Result<(), OwError> {
        let cmd = String::from("e\\t\\b");
        self.t.call(&cmd)?;
        Ok(())
    }

    /// Read Device ID. Reads the ID of the TI device. Wire: `e\t\n`
    pub fn read_device_id(&mut self) -> Result<(), OwError> {
        let cmd = String::from("e\\t\\n");
        self.t.call(&cmd)?;
        Ok(())
    }

    /// Erase Sector. Erases one sector of Flash. Wire: `e\t\e`
    pub fn erase_sector(&mut self, start_address: u32) -> Result<(), OwError> {
        let mut cmd = String::from("e\\t\\e");
        encoding::push_hex(&mut cmd, start_address as u64, 8);
        self.t.call(&cmd)?;
        Ok(())
    }

    /// Write Flash Start. Starts a write operation in flash. Wire: `e\t\t`
    pub fn write_sector_start(&mut self, start_address: u32, length: i32) -> Result<(), OwError> {
        let mut cmd = String::from("e\\t\\t");
        encoding::push_hex(&mut cmd, start_address as u64, 8);
        encoding::push_int(&mut cmd, length as i64);
        self.t.call(&cmd)?;
        Ok(())
    }

    /// Write Flash Send Data. Sends data that for flashing. Wire: `e\t\s`
    pub fn write_sector_send_data(&mut self, hexdata: &[u8]) -> Result<(), OwError> {
        let mut cmd = String::from("e\\t\\s");
        encoding::push_bytes(&mut cmd, hexdata);
        self.t.call(&cmd)?;
        Ok(())
    }

    /// Reset to App. Restarts Micro and runs app if valid. Wire: `e\t\g`
    pub fn reset_to_app(&mut self) -> Result<(), OwError> {
        let cmd = String::from("e\\t\\g");
        self.t.call(&cmd)?;
        Ok(())
    }

    /// Select Active Radio. Selects betwewen Radio 1 and 2. Wire: `e\t\a`
    pub fn toggle_active_radio(&mut self, radio: i32) -> Result<(), OwError> {
        let mut cmd = String::from("e\\t\\a");
        encoding::push_int(&mut cmd, radio as i64);
        self.t.call(&cmd)?;
        Ok(())
    }
}
