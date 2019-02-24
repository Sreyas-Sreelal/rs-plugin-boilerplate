use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use samp_sdk::{log,define_native,natives};

define_native!(foo);

pub struct {{crate_name}};

impl {{crate_name}} {
	pub fn load(&self) -> bool {
		log!("Plugin Loaded!");
		return true;
	}

	pub fn unload(&self) {
		log!("Plugin Unloaded!");
	}

	pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
		let natives = natives!{
			"Foo" => foo
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

impl Default for {{crate_name}} {
	fn default() -> Self {
		{{crate_name}} {
		}
	}
}