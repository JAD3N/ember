#[macro_use]
extern crate log;

pub mod app;

use winit::{
    event::Event,
    event_loop::EventLoop,
    window::Window,
};

fn run(window: Window) {

}

pub fn init() {
    // let event_loop = EventLoop::new();
    // let window = Window::new(&event_loop).unwrap();

    // #[cfg(not(target_arch = "wasm32"))]
    // {
    //     #[cfg(feature = "subscriber")]
    //     wgpu::util::initialize_default_subscriber(None);

    //     futures::executor::block_on(async {
    //         info!("");
    //         loop {}
    //     });
    // }

    // #[cfg(target_arch = "wasm32")]
    // {
    //     console_error_panic_hook::set_once();
    //     console_log::init().expect("couldn't initialize logger");

    //     use winit::platform::web::WindowExtWebSys;

    //     web_sys::window()
    //         .and_then(|win| win.document())
    //         .and_then(|document| document.body())
    //         .and_then(|body| body.append_child(&web_sys::Element::from(window.canvas())).ok())
    //         .expect("couldn't create canvas");

    //     wasm_bindgen_futures::spawn_local(async {
    //         info!("wasm test");
    //     })
    // }
}
