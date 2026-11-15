use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Graphite",
        options,
        Box::new(|_cc| Ok(Box::new(Graphite::default()))),
    )
}

#[derive(Default)]
struct Graphite {
    label: String,
    value: f32,
}

impl eframe::App for Graphite {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Graphite");
        ui.horizontal(|ui| {
            ui.label("text");
            ui.text_edit_singleline(&mut self.label);
        });
        ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
        if ui.button("Increment").clicked() {
            self.value += 1.0;
        }
        ui.label(format!("Hello '{}', value: {}", self.label, self.value));
    }
}