
use log::info;
use samp::plugin::SampPlugin;

#[allow(non_camel_case types)]
pub struct {{crate_name}};

impl SampPlugin for {{crate_name}} {
    fn on_load(&mut self) {
        info!("Loaded!");
    }

    fn on_unload(self: Box<{{crate_name}}>) {
        info!("Unloaded!");
    }
}