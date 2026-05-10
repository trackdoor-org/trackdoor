use maplibre_native::ImageRendererBuilder;
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

struct App {
    window: Option<Arc<Window>>,
    map_renderer: Option<maplibre_native::ImageRenderer<maplibre_native::Continuous>>,
    style_url: String,
    zoom: f64,
    center_lat: f64,
    center_lon: f64,
    bearing: f64,
    pitch: f64,
    is_dragging: bool,
    last_mouse_pos: (f64, f64),
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            map_renderer: None,
            style_url: "https://demotiles.maplibre.org/style.json".to_string(),
            zoom: 2.0,
            center_lat: 0.0,
            center_lon: 0.0,
            bearing: 0.0,
            pitch: 0.0,
            is_dragging: false,
            last_mouse_pos: (0.0, 0.0),
        }
    }

    fn init_renderer(&mut self, window: Arc<Window>) {
        let size = window.inner_size();
        
        let width = NonZeroU32::new(size.width).unwrap_or(NonZeroU32::new(1).unwrap());
        let height = NonZeroU32::new(size.height).unwrap_or(NonZeroU32::new(1).unwrap());

        // Chain the builder calls to avoid move issues
        let renderer = ImageRendererBuilder::new()
            .with_size(width, height)
            .build_continuous_renderer();

        // Load style (returns (), no unwrap needed)
        let url = url::Url::parse(&self.style_url).unwrap();
        
        // Renderer must be mutable to load style
        let mut renderer = renderer; 
        renderer.load_style_from_url(&url);

        self.map_renderer = Some(renderer);
        self.render();
    }

    fn render(&mut self) {
        // Borrow mutably to call render_once
        if let Some(ref mut renderer) = self.map_renderer {
            renderer.render_once();
        }
    }

    fn update_camera(&mut self) {
        if let Some(ref mut renderer) = self.map_renderer {
            // Arguments: (latitude, longitude, zoom, bearing, pitch)
            // Note: The order in 0.4.5 is often (lat, lon, zoom, bearing, pitch) or similar.
            // Based on the error "expected f64, found Longitude", it expects raw f64.
            // Let's try the standard order: lat, lon, zoom, bearing, pitch
            // If this is wrong, check the docs for the exact order: 
            // set_camera(lat: f64, lon: f64, zoom: f64, bearing: f64, pitch: f64)

            renderer.set_camera(
                maplibre_native::Latitude(self.center_lat),
                maplibre_native::Longitude(self.center_lon),
                self.zoom,
                self.bearing,
                self.pitch,
            );
            self.render();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_attrs = Window::default_attributes()
            .with_title("MapLibre Native + Winit (v0.4.5)")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

        let window = event_loop.create_window(window_attrs).expect("Failed to create window");
        let window = Arc::new(window);
        self.window = Some(window.clone());
        window.request_redraw();
        
        self.init_renderer(window);
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(_physical_size) => {
                // Resize handling might require rebuilding the renderer in 0.4.5
                // For now, just re-render to see if it updates
                self.render();
            }
            WindowEvent::CloseRequested => {
                _event_loop.exit();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left {
                    self.is_dragging = state == winit::event::ElementState::Pressed;
                    if self.is_dragging {
                        self.last_mouse_pos = (0.0, 0.0); 
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.is_dragging {
                    if self.last_mouse_pos.0 != 0.0 || self.last_mouse_pos.1 != 0.0 {
                        let delta_x = position.x - self.last_mouse_pos.0;
                        let delta_y = position.y - self.last_mouse_pos.1;

                        // Simple pan approximation
                        let meters_per_pixel = 156543.0 * 0.5_f64.powf(-self.zoom);
                        let deg_per_meter = 1.0 / 111000.0;

                        let dx_deg = -delta_x * meters_per_pixel * deg_per_meter;
                        let dy_deg = delta_y * meters_per_pixel * deg_per_meter;

                        self.center_lon += dx_deg;
                        self.center_lat += dy_deg;

                        self.update_camera();
                    }
                    self.last_mouse_pos = (position.x, position.y);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let factor = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, dy) => dy as f64 * 0.1,
                    winit::event::MouseScrollDelta::PixelDelta(delta) => delta.y as f64 * 0.001,
                };
                
                self.zoom = (self.zoom + factor).max(0.0).min(20.0);
                self.update_camera();
            }
            _ => {}
        }
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    
    event_loop.run_app(&mut app).expect("Event loop failed");
}
