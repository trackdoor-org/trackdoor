use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Trackdoor", native_options, Box::new(|cc| Ok(Box::new(TrackdoorApp::new(cc)))));
}

#[derive(Default)]
struct TrackdoorApp {}

impl TrackdoorApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for TrackdoorApp {
   fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show_inside(ui, |ui| {
           ui.heading("Hello World!");
       });
   }
}
