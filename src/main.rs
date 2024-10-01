use eframe::egui;

struct RafCompressor {
    files: Vec<String>,
}

impl Default for RafCompressor {
    fn default() -> Self {
        Self {
            files: Vec::new(),
        }
    }
}

impl RafCompressor {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for RafCompressor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Liten: Compress Your RAF Files");
            
            if ui.button("Select RAF files").clicked() {
                if let Some(paths) = rfd::FileDialog::new().pick_files() {
                    self.files = paths.iter().filter_map(|path| path.to_str().map(String::from)).collect();
                }
            }

            ui.label(format!("Selected files: {}", self.files.len()));

            if ui.button("Compress and Convert").clicked() {
                // TODO: Implement compression and conversion
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RAF Compressor",
        options,
        Box::new(|cc| Ok(Box::new(RafCompressor::new(cc))))
    )
}