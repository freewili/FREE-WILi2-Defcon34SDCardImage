//! Apps functions menu - generated from fwMenuApps. Do not edit.
//! Each method packs its args into a Vec<u8> and calls the host
//! import `ow_call(cmd_index, args, args_len, ret, ret_cap)` via
//! crate::transport::call - the firmware assembles/decodes the
//! wire command natively; there is no encoding/framing here.

use crate::transport::OwError;

pub struct Apps2<'a> {
    #[allow(dead_code)]
    pub(crate) t: &'a mut (),
}

impl<'a> Apps2<'a> {
    /// Launch App. Wire: `a\a`
    pub fn launch_app(&mut self) -> Result<(), OwError> {
        let a = crate::transport::Args::new();
        let _r = crate::transport::call(167 /* CMD_APPS_LAUNCH_APP */, &a)?;
        Ok(())
    }
}
