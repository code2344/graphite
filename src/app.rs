use eframe::egui;


#[derive(Default)]
pub struct Graphite {
    label: String,
    value: f32,
}

impl eframe::App for Graphite {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("toolbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Graphite");
                ui.separator();
                ui.button("Undo");
                ui.button("Redo");
            });
        });
        
    }
}