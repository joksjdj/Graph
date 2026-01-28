use colored::*;
use std::time::Instant;

pub struct Time;
impl Time {
    pub fn run() -> impl Fn(fn()) {
        move |f: fn()| {
            let start = Instant::now();
            println!("\n");
            f();
            let duration = start.elapsed();
            let ms = (duration.as_secs_f64() * 1000.0).ceil() / 1000.0;
            println!("Time elapsed: {:.3} ms\n", ms);
        }
    }
}

pub fn print_test(text: &str) {
    println!("{}", text.to_string().purple());
}

pub struct GraphData {
    pub x: f32,
    pub y: f32,
    pub multiplier: f32,
    pub power_of: f32,
}
impl GraphData {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            multiplier: 1.0,
            power_of: 1.0,
        }
    }
    
    pub fn run(&mut self, ui: &mut egui::Ui) {
        ui.label("GRAPH");

        let (rect, _response) = ui.allocate_exact_size(
            egui::vec2(300.0, 200.0),
            egui::Sense::hover(),
        );

        let painter = ui.painter_at(rect);

        let painter_arr = [
            [rect.left_top(), rect.left_bottom()],
            [rect.left_bottom(), rect.right_bottom()],
        ];

        for line in painter_arr {
            painter.line_segment(
                line,
                egui::Stroke::new(2.0, egui::Color32::WHITE),
            );
        }

        let mut points = Vec::new(); 
        for i in 0..300 { 
            let x = rect.left() + i as f32; 
            let y = rect.top() + 100.0 + (i as f32 * 0.1).sin() * 40.0; // uneven curve 
            points.push(egui::pos2(x, y)); 
        } 
        painter.add(egui::Shape::line( points, egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE), ));

        if ui.button("Linear").clicked() {
            println!("Button clicked!");
        }
        if ui.button("Jagged").clicked() {
            println!("Button clicked!");
        }
        if ui.button("Apply").clicked() {
            println!("Button clicked!");
        }
    }
}