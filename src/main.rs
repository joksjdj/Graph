use graph::{print_test, Time, GraphData};
use eframe::egui;

fn main() {
    let t = Time::run();
    t(|| print_test("Hello from lib.rs"));

    if let Err(err) = eframe::run_native(
        "My App",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default()))
    ) {
        eprintln!("eframe error: {err}");
    }
}

pub struct MyApp { 
    pub graph: GraphData, 
}

impl Default for MyApp {
    fn default() -> Self { 
        Self { 
            graph: GraphData::new(), 
        } 
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.graph.run(ui); 
        });
    }
}