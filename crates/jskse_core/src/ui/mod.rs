use windows::Win32::Graphics::{
    Direct3D11::{ID3D11Device, ID3D11DeviceContext, ID3D11RenderTargetView},
    Dxgi::IDXGISwapChain,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

pub unsafe fn initialize_ui() {}

struct JskseUi {
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
    swap_chain: IDXGISwapChain,
    render_target: Option<ID3D11RenderTargetView>,
    egui_ctx: egui::Context,
    egui_renderer: egui_directx11::Renderer,
    egui_winit: egui_winit::State,
}

trait UiWindow: Sized {
    fn on_event(&mut self, window: &Window, event: &WindowEvent);
    fn new(window: &Window) -> Self;
    fn run(window_attributes: WindowAttributes) {
        struct Runner<T: UiWindow> {
            window_attributes: WindowAttributes,
            window: Option<Window>,
            app: Option<T>,
        }

        impl<T: UiWindow> ApplicationHandler for Runner<T> {
            fn resumed(&mut self, event_loop: &ActiveEventLoop) {
                let window = event_loop
                    .create_window(self.window_attributes.clone())
                    .expect("Failed to create window");
                self.app = Some(T::new(&window));
                self.window = Some(window);
            }

            fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {}

            fn window_event(
                &mut self,
                event_loop: &ActiveEventLoop,
                window_id: winit::window::WindowId,
                event: WindowEvent,
            ) {
            }
        }

        EventLoop::new()
            .expect("Failed to create event loop")
            .run_app(&mut Runner::<Self> {
                window_attributes,
                window: None,
                app: None,
            })
            .expect("Failed to run event loop");
    }
}
