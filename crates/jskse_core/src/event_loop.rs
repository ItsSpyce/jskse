// We have to roll our own event loop because Tokio uses MSVC incompatible libraries.
// It should only run on the main thread and do X amount of work in between each poll.
// Since SKSE doesn't expose any OnTick events, we need to loop through and do work
// then hand off access back to the main thread.

use std::task::Poll;

use deno_core::JsRuntime;

pub struct SKSEEventLoop<'a> {
    pub(crate) js_runtime: &'a mut JsRuntime,
}

impl<'a> SKSEEventLoop<'a> {
    pub fn new(js_runtime: &'a mut JsRuntime) -> Self {
        Self { js_runtime }
    }

    pub fn run(&self) {
        loop {
            match self
                .js_runtime
                .poll_event_loop(self.js_runtime.main_context(), Default::default())
            {
                Poll::Ready(val) => {}
                Poll::Pending => {
                    // wait some more
                }
            }
        }
    }
}
