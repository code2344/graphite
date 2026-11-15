mod app;
use app::Graphite;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Graphite",
        options,
        Box::new(|_cc| Ok(Box::new(Graphite::default()))),
    )
}

