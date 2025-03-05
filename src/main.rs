use std::time;

use eframe::egui;
use timebase::Event;

mod timebase;



#[derive(Default)]
struct DemoApp {
    button: timebase::Input
}



impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {


        let f = timebase::Frame::new(time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_millis() as i32);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Timebase demo thing");

            if ui.button("Trigger").clicked() {
                f.trigger(&mut self.button);
            }

            let tone = f.timebase(timebase::TimebaseMode::Repeat, 200, Event::zero()).sin();

            let ramp = f.timebase(timebase::TimebaseMode::OneShot, 2000, self.button.into()).scale(100.0, 0.0);

            let mut toneval = tone * ramp;
            ui.add(egui::Slider::new(&mut toneval, 0f32..=100f32));

            let twosec = f.timebase(timebase::TimebaseMode::OneShot, 2000, self.button.into());

            let evs = twosec.fountain::<10>(0, 8);

            for ev in evs {
                let pew = f.timebase(timebase::TimebaseMode::OneShot, 200, ev);
                let mut val = pew.scale(0f32, 100f32);
                ui.add(egui::Slider::new(&mut val, 0f32..=100f32));
            }

            let mut dist = 0;
            for i in 0..10 {
                let pew = f.timebase(timebase::TimebaseMode::OneShot, 400, twosec.sync());
                let mut val = pew.shift(dist).sin() * 100f32;
                ui.add(egui::Slider::new(&mut val, 0f32..=100f32));
                dist += 100;
            }


            {
                let mut val = twosec.scale(0f32,100f32);
                ui.add(egui::Slider::new(&mut val, 0f32..=100f32));
            }

            let mut w = twosec.wave([10.0, 20.0, 30.0, 80.0]);
            ui.add(egui::Slider::new(&mut w, 0f32..=100f32));

        });

        ctx.request_repaint();
    }
}



fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0,640.0]),
        ..Default::default()
    };

    eframe::run_native("Demo App", options, Box::new(|cc| {
        Ok(Box::<DemoApp>::default())
    }));
}