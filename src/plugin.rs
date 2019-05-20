use log::info;
use samp::plugin::SampPlugin;


pub struct Plugin;

impl SampPlugin for Plugin {
    fn on_load(&mut self) {
        info!("{{crate_name}} Loaded!");
    }

    fn on_unload(self: Box<Plugin>) {
        info!("{{crate_name}} Unloaded!");
    }
}