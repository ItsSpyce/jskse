use boa_engine::Context;

pub mod console;

static GLOBAL_MODULES: &[fn(&Context)] = &[console::register_console];

pub fn register_globals(context: &Context) {
    for module in GLOBAL_MODULES {
        module(context);
    }
}
