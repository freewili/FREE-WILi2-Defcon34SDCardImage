//! Scripting Functions menu - generated from fwMenuScripting. Do not edit.
//! Each method packs its args into a Vec<u8> and calls the host
//! import `ow_call(cmd_index, args, args_len, ret, ret_cap)` via
//! crate::transport::call - the firmware assembles/decodes the
//! wire command natively; there is no encoding/framing here.

use crate::transport::OwError;

pub struct Scripting2<'a> {
    #[allow(dead_code)]
    pub(crate) t: &'a mut (),
}

impl<'a> Scripting2<'a> {
    /// ZoomIO Functions sub-menu.
    pub fn zoom_io(self) -> super::zoom_io::ZoomIo<'a> {
        super::zoom_io::ZoomIo { t: self.t }
    }

    /// Launch Script. Wire: `s\a`
    pub fn launch_script(&mut self) -> Result<(), OwError> {
        let a = crate::transport::Args::new();
        let _r = crate::transport::call(159 /* CMD_SCRIPTING_LAUNCH_SCRIPT */, &a)?;
        Ok(())
    }
}
