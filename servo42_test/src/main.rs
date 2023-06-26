use std::{time::Duration, thread::sleep};

use eframe::egui;
use motor::servo42::Servo42C;
use serial::standard::{serialport, serialport::*};
fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.vsync=true;
    let _ =eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

struct MyEguiApp {
    m: Servo42C<Box<dyn SerialPort>>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback
        let s = serialport::new("/dev/ttyACM0", 115_200)
        .timeout(Duration::from_millis(1000))
        .parity(Parity::None)
        .baud_rate(38400)
        .stop_bits(serial::standard::StopBits::One)
        .flow_control(serial::standard::FlowControl::None)
        .open().expect("Failed to open port");
        let mut m = Servo42C { address: 0xe0, s };
        let _ =m.set_microstep(150);
        let _ =m.set_en_active(0);
        let _=m.set_enable(true);
        let _=m.set_subdivision_interpolation(true);
        let _=m.set_lock(false);
        //let _ =m.set_speed(false, 50);
        let _ =m.goto(50, 50);
        //m.calibrate().unwrap();
        let t= MyEguiApp{m};
        t
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
        let z = format!("{:?}", self.m.read_encoder_value());
           ui.heading(&z );
       });
       let _ =self.m.goto(178, 1);
       sleep(Duration::from_secs(1));
       let _ =self.m.goto(50, 1);
       sleep(Duration::from_secs(1));
       /*if let Ok(_t) = self.m.goto(50, 1000){
            
       }*/
       
       ctx.request_repaint();
   }
}