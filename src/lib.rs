#[macro_use]
extern crate samp_sdk;
//extern crate your_external_crates

mod plugin;
mod natives;

use plugin::{{crate_name}};

new_plugin!({{crate_name}});

