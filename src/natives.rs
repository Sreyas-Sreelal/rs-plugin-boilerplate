use samp::prelude::*;
use samp::native;
use samp::error::AmxResult;

impl super::{{crate_name| pascal_case}} {
    #[native(name = "Foo")]
    pub fn foo(&mut self, _amx: &Amx) -> AmxResult<bool> {
        Ok(true)
    }
}
