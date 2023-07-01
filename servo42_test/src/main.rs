use std::time::Duration;

use eframe::egui::{self};
use motor::servo42::Servo42C;
use serial::standard::{serialport, serialport::*};
fn main() {
    let native_options = eframe::NativeOptions::default();
    //native_options.vsync=true;
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

struct MyEguiApp {
    m: Option<Servo42C<Box<dyn SerialPort>>>,
    name: String,
    is_connect: bool,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        /*
        let mut m = <Servo42C<SerialTest>>::default();
        let _ =m.set_microstep(16);
        let _ =m.set_zero();
        let _=m.set_subdivision_interpolation(true);
        let _=m.set_lock(false);
        /*let _=m.set_acc(None);
        m.set_kp(None);
        m.set_ki(None);
        m.set_kd(None);*/
        //let _ =m.set_speed(false, 50);
        //let _ =m.set_speed(true, 2);
        let _ = m.goto(127, 10000000);
        //let _ =m.stop();

        let mut average=0.0;
        let mut t=SystemTime::now();
        sleep(Duration::from_millis(100000));
        loop{
            m.read_encoder_value();
            let fps=1_000_000.0f64/t.elapsed().unwrap().as_micros() as f64;
            average=(average*99.0+fps)/100.0;
            t=SystemTime::now();
            //println!("{}", average);
        }*/
        //m.calibrate().unwrap();*/
        MyEguiApp {
            m: Default::default(),
            name: "/dev/ttyACM0".to_string(),
            is_connect: false,
        }
    }
}

macro_rules! cmd {
    ( $func:ident, $value:ident) => {
        fn $func(&mut self, ui: &mut egui::Ui) {
            ui.horizontal(|ui| {
                ui.heading(stringify!($value));
                //self.m.$value.MIN;
                if let Some(m) = &mut self.m {
                    ui.add(egui::Slider::new(&mut m.$value, u16::MIN..=u16::MAX));
                    if ui.add(egui::Button::new("Send")).clicked() {
                        let _ = m.$func(m.$value);
                    }
                }
            });
        }
    };
}
impl MyEguiApp {
    cmd!(set_kp, kp);
    cmd!(set_ki, ki);
    cmd!(set_kd, kd);
    cmd!(set_acc, acc);
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.name));
                    if ui
                        .add(egui::Checkbox::new(&mut self.is_connect, "Connect"))
                        .changed()
                    {
                        if self.is_connect {
                            println!("connecting");
                            if let Ok(val) = serialport::new("/dev/ttyACM0", 115_200)
                                .timeout(Duration::from_millis(1000))
                                .parity(Parity::None)
                                .baud_rate(115200)
                                .stop_bits(serial::standard::StopBits::Two)
                                .flow_control(serial::standard::FlowControl::None)
                                .open()
                            {
                                let m = Servo42C::<Box<dyn SerialPort>> {
                                    address: 0xe0,
                                    s: val,
                                    kp: 0x650,
                                    ki: 0x120,
                                    kd: 0x650,
                                    acc: 0x11e,
                                };
                                self.m = Some(m);
                            } else {
                                self.is_connect = false;
                            }
                        } else {
                            self.m = None;
                            println!("disconnecting");
                        }
                    }
                });
                self.set_kp(ui);
                self.set_ki(ui);
                self.set_kd(ui);
                self.set_acc(ui);
            });
        });

        //let _ =self.m.goto(178, 1);
        //sleep(Duration::from_secs(1));
        /*let t =self.m.goto(50, 1);
        if t==2{
             println!("WTF");
        }
        sleep(Duration::from_secs(1));*/
        /*if let Ok(_t) = self.m.goto(50, 1000){

        }*/

        ctx.request_repaint();
    }
}
