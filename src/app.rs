use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
    dpi::PhysicalSize,
};

pub struct App {
    window: Window,
    event_loop: EventLoop<()>,
    instance: wgpu::Instance,
    size: PhysicalSize<u32>,

    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl App {
    async fn new() -> Self {
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
        let window = winit::window::WindowBuilder::new()
            .with_title("Test Title")
            .build(&event_loop)
            .unwrap();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(&window);

            (size, surface)
        };

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        }).await.unwrap();

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

        Self {
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

    fn start(App {
        window,
        event_loop,
        instance,
        size,
        surface,
        adapter,
        device,
        queue,
    }: App) {
        event_loop.run(move |event, _, control_flow| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => println!("close me!"),
                    _ => {},
                }
            }
        });
    }

    pub fn run() {
        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                let app = Self::new().await;
                Self::start(app);
            });
        };

        #[cfg(not(target_arch = "wasm32"))]
        {
            let app = futures::executor::block_on(Self::new());
            Self::start(app);
        };
    }
}