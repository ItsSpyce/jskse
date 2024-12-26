const { op_console_log, op_console_error, op_console_warn, op_console_debug } =
  Deno.core;

globalThis.console = {
  log: (...args) =>
    op_console_log(args.map((arg) => JSON.stringify(arg)).join(" ")),
  error: (...args) =>
    op_console_error(args.map((arg) => JSON.stringify(arg)).join(" ")),
  warn: (...args) =>
    op_console_warn(args.map((arg) => JSON.stringify(arg)).join(" ")),
  debug: (...args) =>
    op_console_debug(args.map((arg) => JSON.stringify(arg)).join(" ")),
};
