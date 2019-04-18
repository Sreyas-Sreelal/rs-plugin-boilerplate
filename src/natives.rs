use samp::prelude::*;
use samp::native;
use samp::error::AmxResult;

impl super::{{crate_name}} {
    #[native(name = "foo")]
    pub fn foo(&mut self,amx: &Amx) -> AmxResult<bool> {
        Ok(true)
    }
}