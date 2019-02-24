use samp_sdk::types::Cell;
use samp_sdk::amx::{AmxResult, AMX};

impl super::{{crate_name}}{
	pub fn foo(&mut self,_:&AMX) -> AmxResult<Cell> {
		Ok(1)
	}
}
