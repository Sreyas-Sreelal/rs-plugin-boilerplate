mod natives;
mod plugin;

use crate::plugin::{{crate_name | pascal_case}};
use samp::initialize_plugin;

initialize_plugin!(
    natives: [
        {{crate_name | pascal_case}}::foo
    ],
    {
        samp::plugin::enable_process_tick();
        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);

        let log_file = fern::log_file("{{crate_name | pascal_case}}.log").expect("Cannot create log file!");

        let trace_level = fern::Dispatch::new()
            .level(log::LevelFilter::Trace)
            .chain(log_file);

        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[{{crate_name | pascal_case}}] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .chain(trace_level)
            .apply();

        {{crate_name | pascal_case}} {
        }
    }
);