use deno_core::op2;

#[op2(fast)]
pub fn op_console_log(#[string] msg: String) {
    log::info!("{}", msg);
}

#[op2(fast)]
pub fn op_console_error(#[string] msg: String) {
    log::error!("{}", msg);
}

#[op2(fast)]
pub fn op_console_warn(#[string] msg: String) {
    log::warn!("{}", msg);
}

#[op2(fast)]
pub fn op_console_debug(#[string] msg: String) {
    log::debug!("{}", msg);
}
