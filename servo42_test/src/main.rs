use std::{
    time::{Duration, SystemTime},
    vec,
};

use eframe::egui::{self};
use motor::prelude::*;
use serial::standard::{serialport, serialport::*};
fn main() {
    let _ = eframe::run_native(
        "My egui App",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

struct MyEguiApp {
    m: Option<Servo42C<Box<dyn SerialPort>>>,
    name: String,
    is_connect: bool,
    encoder: Vec<(f32, f64)>,
    errors: Vec<(f32, i64)>,
    correct: usize,
    invalid: usize,
    t: SystemTime,
    start: SystemTime,
    dir: bool,

    
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
            encoder: vec![],
            errors: vec![],
            correct: 5,
            invalid: 0,
            t: SystemTime::now(),
            start: SystemTime::now(),
            dir: true,
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

impl MyEguiApp {
    fn plot(&self, ui: &mut egui::Ui) -> egui::Response {
        use egui::plot::{Line, PlotPoints};
        let mut cont = 1u32;
        let encoder: PlotPoints = self
            .encoder
            .iter()
            .map(|(x, y)| {
                cont += 1;
                let y = *y / 65536.0;
                [*x as f64, y]
            })
            .collect();
        let errors: PlotPoints = self
            .errors
            .iter()
            .map(|(x, y)| {
                cont += 1;
                //let y = (*y as f64);
                [*x as f64, (*y as f64) / 3000.0]
            })
            .collect();
        let encoder = Line::new(encoder);
        let errors = Line::new(errors);
        egui::plot::Plot::new("example_plot")
            .height(200.0)
            //.data_aspect(cont as f32)
            .show(ui, |plot_ui| {
                plot_ui.line(encoder);
                plot_ui.line(errors);
            })
            .response
    }
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
                            if let Ok(val) = serialport::new(&self.name, 38_400)
                                .timeout(Duration::from_millis(10))
                                .parity(Parity::None)
                                //.baud_rate(115200)
                                .stop_bits(serial::standard::StopBits::One)
                                .data_bits(DataBits::Eight)
                                .flow_control(serial::standard::FlowControl::None)
                                .open()
                            {
                                let mut m: Servo42C<Box<dyn SerialPort>> =
                                    Servo42C::<Box<dyn SerialPort>>::new(val).unwrap();
                                let _ = m.read_encoder_value().unwrap();
                                //m.goto(138, enc as u32);
                                //while let Err(_) = m.read::<u8>(){};
                                //let _: u8 = m.read().unwrap();
                                //let _ = m.reset();
                                //let _ = m.set_zero_mode(2);
                                //let _ = m.set_zero();
                                self.m = Some(m);
                                self.start = SystemTime::now();
                                self.encoder = vec![];
                                self.correct = 5;
                                self.invalid = 0;
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
                self.plot(ui);
                if self.t.elapsed().unwrap() > Duration::from_secs(3) {
                    self.t = SystemTime::now();
                    self.dir = !self.dir;
                    if let Some(motor) = &mut self.m {
                        let _ = motor.set_microstep(16);
                        let _ = motor.set_speed(10);
                    }
                }
                ui.heading(format!(
                    "correct={} invalid={} rap={} cps={}",
                    self.correct,
                    self.invalid,
                    self.correct as f32 / (self.invalid + self.correct) as f32,
                    (self.invalid + self.correct) as f32
                        / self.start.elapsed().unwrap().as_secs_f32()
                ));
                let data = if let Some(motor) = &mut self.m {
                    if let Ok(val) = motor.read_encoder_value() {
                        self.correct += 1;
                        //println!("{val:?}");
                        Some((self.start.elapsed().unwrap().as_secs_f32(), val))
                    } else {
                        self.invalid += 1;
                        None
                    }
                } else {
                    None
                };
                if let Some(x) = data {
                    println!("{x:?}");
                    self.encoder.push(x);
                }
                let data = if let Some(motor) = &mut self.m {
                    if let Ok(val) = motor.read_error() {
                        self.correct += 1;
                        println!("{val:?}");
                        Some((self.start.elapsed().unwrap().as_secs_f32(), val as i64))
                    } else {
                        self.invalid += 1;
                        None
                    }
                } else {
                    None
                };
                if let Some(x) = data {
                    //println!("{x:?}");
                    self.errors.push(x);
                }
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
