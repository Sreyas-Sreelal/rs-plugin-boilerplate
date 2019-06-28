use log::info;
use samp::plugin::SampPlugin;

pub struct {{crate_name| pascal_case}};

impl SampPlugin for {{crate_name| pascal_case}} {
    fn on_load(&mut self) {
        info!("{{crate_name}} Loaded!");
    }

    fn on_unload(self: Box<{{crate_name | pascal_case}}>) {
        info!("{{crate_name}} Unloaded!");
    }
}
