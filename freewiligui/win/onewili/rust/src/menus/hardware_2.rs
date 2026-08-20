//! Hardware Functions menu - generated from fwMenuHardware. Do not edit.
//! Each method packs its args into a Vec<u8> and calls the host
//! import `ow_call(cmd_index, args, args_len, ret, ret_cap)` via
//! crate::transport::call - the firmware assembles/decodes the
//! wire command natively; there is no encoding/framing here.

use crate::transport::OwError;

pub struct Hardware2<'a> {
    #[allow(dead_code)]
    pub(crate) t: &'a mut (),
}

impl<'a> Hardware2<'a> {
    /// System Functions sub-menu.
    pub fn system(self) -> super::system::System<'a> {
        super::system::System { t: self.t }
    }

    /// Settings. Wire: `h\s`
    pub fn do_something(&mut self) -> Result<(), OwError> {
        let a = crate::transport::Args::new();
        let _r = crate::transport::call(124 /* CMD_HARDWARE_DO_SOMETHING */, &a)?;
        Ok(())
    }
}
