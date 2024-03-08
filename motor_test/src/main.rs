use std::{
    sync::mpsc::{Receiver, Sender},
    vec,
};

use crate::motor_thread::new_thread;
use eframe::egui;
use egui_plot::Points;
use motor::prelude::*;
use motor_thread::{EmptySerial, MotorComand, MotorState};
use serial::standard::{serialport, SerialPort};
pub mod motor_thread;
fn main() {
    let native_options = eframe::NativeOptions::default();
    //native_options.vsync = false;
    let _ = eframe::run_native(
        "",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

struct MyEguiApp {
    name: String,
    baudrate: BaudRate,
    connection: Option<(Receiver<MotorState>, Sender<MotorComand>)>,
    connection_checkbox: bool,
    pos_history: Vec<f64>,
    obj_history: Vec<f64>,
    time_history: Vec<f64>,
    error_history: Vec<f64>,
    cmd_rate: f64,
    motor_builder: Servo42LinearAccBuilder<Box<dyn SerialPort>>,
    arrivato: bool,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        MyEguiApp {
            name: "/dev/ttyACM0".to_string(),
            baudrate: BaudRate::B115200,
            connection: None,
            connection_checkbox: false,
            pos_history: vec![],
            obj_history: vec![],
            time_history: vec![],
            error_history: vec![],
            cmd_rate: 0.,
            motor_builder: Servo42LinearAccBuilder::new(Box::new(EmptySerial {})),
            arrivato: false,
        }
    }
}
/*
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
}*/

macro_rules! build_parameters {
    ($ui:ident, $self:ident, $name:ident) => {
        $ui.horizontal(|ui| {
            ui.heading(stringify!($name));
            ui.add(egui::Slider::new(
                &mut $self.motor_builder.$name,
                0.0..=32.0,
            ));
        })
    };
}

impl MyEguiApp {
    fn plot(&self, ui: &mut egui::Ui) {
        use egui_plot::{Line, PlotPoints};
        let pos: PlotPoints = self
            .time_history
            .iter()
            .zip(self.pos_history.iter())
            .map(|(x, y)| [*x, *y])
            .collect();
        let obj: PlotPoints = self
            .time_history
            .iter()
            .zip(self.obj_history.iter())
            .map(|(x, y)| [*x, *y])
            .collect();
        let errors: PlotPoints = self
            .time_history
            .iter()
            .zip(self.error_history.iter())
            .map(|(x, y)| [*x, *y])
            .collect();
        let mut prev = 0.;
        let mut different = 0;

        for x in self.pos_history.iter() {
            if prev != *x {
                different += 1;
            }
            prev = *x;
        }
        ui.label(format!("{different}"));
        let errors = Line::new(errors);
        let pos = Line::new(pos);
        let obj = Points::new(obj);
        egui_plot::Plot::new("example_plot")
            .height(200.0)
            //.data_aspect(cont as f32)
            .show(ui, |plot_ui| {
                plot_ui.line(pos);
                plot_ui.points(obj);
                plot_ui.line(errors);
            });
        ui.add(egui::Label::new(format!("cmd_speed={:?}", self.cmd_rate)));
    }

    ///Function for display all connection settings
    fn connection_settings(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_source(0)
                .selected_text(&self.name)
                .show_ui(ui, |ui| {
                    let ports = serialport::available_ports().expect("No ports found!");
                    for x in ports {
                        ui.selectable_value(&mut self.name, x.port_name.clone(), &x.port_name);
                    }
                });

            egui::ComboBox::from_id_source(1)
                .selected_text(format!("{:?}", self.baudrate))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.baudrate, BaudRate::B9600, "9600");
                    ui.selectable_value(&mut self.baudrate, BaudRate::B19200, "19200");
                    ui.selectable_value(&mut self.baudrate, BaudRate::B25000, "25000");
                    ui.selectable_value(&mut self.baudrate, BaudRate::B38400, "38400");
                    ui.selectable_value(&mut self.baudrate, BaudRate::B57600, "57600");
                    ui.selectable_value(&mut self.baudrate, BaudRate::B115200, "115200");
                });

            if ui
                .add(egui::Checkbox::new(
                    &mut self.connection_checkbox,
                    "Connect",
                ))
                .changed()
            {
                if self.connection_checkbox {
                    //if want to connect
                    println!("connecting");
                    match new_thread(
                        &self.name.to_string(),
                        self.baudrate.clone().into(),
                        &self.motor_builder,
                    ) {
                        Ok(conn) => {
                            self.connection = Some(conn);
                            self.connection_checkbox = true;
                            self.obj_history = Vec::new();
                            self.pos_history = Vec::new();
                            self.time_history = Vec::new();
                            self.error_history = Vec::new();
                        }
                        Err(err) => {
                            println!("impossible to connect: {:?}", err);
                            self.connection_checkbox = false;
                            self.connection = None;
                        }
                    }
                } else {
                    println!("disconnecting");
                    //if want to disconnect
                    if let Some((_, conn)) = &self.connection {
                        let _ = conn.send(MotorComand::KillThread);
                    }
                    self.connection_checkbox = false;
                    self.connection = None;
                }
            }
        });
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //if we are connected
            if let Some((reader, _)) = &self.connection {
                //load all available data
                reader.try_iter().for_each(|val| {
                    self.obj_history.push(val.obbiettivo);
                    self.pos_history.push(val.pos);
                    self.error_history.push(val.error);
                    self.time_history.push(val.timing.as_secs_f64());
                    self.cmd_rate = val.cmd_rate;
                    self.arrivato = val.reached;
                });
            }
            ui.add(egui::Checkbox::new(&mut self.arrivato, "Arrivato"));
            ui.vertical(|ui| {
                self.connection_settings(ui);
                build_parameters!(ui, self, acc);
                build_parameters!(ui, self, max_speed);
                build_parameters!(ui, self, max_err);
                build_parameters!(ui, self, precision);
                self.plot(ui);
            });
        });

        ctx.request_repaint();
    }
}
