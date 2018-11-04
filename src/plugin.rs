use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;

/*
export native functions

define_native!(foo);
*/

pub struct MyPlugin;

impl MyPlugin {
	pub fn load(&self) -> bool {
		log!("Plugin Loaded!");
		return true;
	}

	pub fn unload(&self) {
		log!("Plugin Unloaded!");
	}

	pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
		let natives = natives!{
			/*
			"YourNativeFunctionName" => FunctionName,
			*/
		};

		match amx.register(&natives) {
			Ok(_) => log!("Natives are successful loaded"),
			Err(err) => log!("Whoops, there is an error {:?}", err),
		}

		AMX_ERR_NONE
	}

	pub fn amx_unload(&self, _: &mut AMX) -> Cell {
		AMX_ERR_NONE
	}

}

impl Default for MyPlugin {
	fn default() -> Self {
		MyPlugin {
		}
	}
}