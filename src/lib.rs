#[macro_use]
extern crate log;

mod app;
mod ember;
mod scene;

pub use app::*;
pub use ember::*;
pub use scene::*;

use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
    dpi::PhysicalSize,
};

struct Setup {
    window: Arc<Window>,
    event_loop: EventLoop<()>,
    instance: wgpu::Instance,
    size: PhysicalSize<u32>,

    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

async fn setup<A: App>() -> Setup {
    #[cfg(feature = "subscriber")]
    {
        let chrome_tracing_dir = std::env::var("WGPU_CHROME_TRACE");
        wgpu::util::initialize_default_subscriber(
            chrome_tracing_dir.as_ref().map(std::path::Path::new).ok(),
        );
    };

    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        console_log::init().expect("couldn't initialize logger");
    };

    let event_loop = EventLoop::new();
    let window = Arc::new(Window::new(&event_loop).unwrap());

    #[cfg(not(target_arch = "wasm32"))]
    let (mut pool, spawner) = {
        let local_pool = futures::executor::LocalPool::new();
        let spawner = local_pool.spawner();

        (local_pool, spawner)
    };

    #[cfg(target_arch = "wasm32")]
    let spawner = {
        use futures::{future::LocalFutureObj, task::SpawnError};
        use winit::platform::web::WindowExtWebSys;

        struct WebSpawner {}

        impl LocalSpawn for WebSpawner {
            fn spawn_local_obj(
                &self,
                future: LocalFutureObj<'static, ()>,
            ) -> Result<(), SpawnError> {
                Ok(wasm_bindgen_futures::spawn_local(future))
            }
        }

        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");

        WebSpawner {}
    };

    info!("Creating instance...");

    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let (size, surface) = unsafe {
        let size = window.inner_size();
        let surface = instance.create_surface(window.as_ref());

        (size, surface)
    };

    info!("Requesting adapater...");

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
    }).await.unwrap();

    info!("Requesting device...");

    let trace_dir = std::env::var("WGPU_TRACE");
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            shader_validation: true,
        },
        trace_dir.ok()
            .as_ref()
            .map(std::path::Path::new),
    ).await.unwrap();

    info!("Completed setup!");

    Setup {
        window,
        event_loop,
        instance,
        size,
        surface,
        adapter,
        device,
        queue,
    }
}

fn start<A: App>(
    Setup {
        window,
        event_loop,
        instance,
        size,
        surface,
        adapter,
        device,
        queue,
    }: Setup,
) {
    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: if cfg!(target_arch = "wasm32") {
            wgpu::TextureFormat::Bgra8Unorm
        } else {
            wgpu::TextureFormat::Bgra8UnormSrgb
        },
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let app = A::new(Ember::new(window.clone()));

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => println!("close me!"),
                _ => {},
            }
        }
    });
}

pub fn run<A: App>() {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            let setup = setup::<A>().await;
            start::<A>(setup);
        });
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        let setup = futures::executor::block_on(setup::<A>());
        start::<A>(setup);
    };
}
