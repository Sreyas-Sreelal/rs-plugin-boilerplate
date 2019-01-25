#[macro_use]
extern crate samp_sdk;

mod plugin;
mod natives;

use plugin::{{crate_name}};

new_plugin!({{crate_name}});

