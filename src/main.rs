mod robot;
mod task;
mod bot_type;
mod bot_manager;

use crate::bot_type::BotType;
use crate::robot::Robot;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Robot App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
    bot_type: BotType,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            bot_type: BotType::Aeronautical,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bot-O-Matic");
            ui.horizontal(|ui| {
                ui.label("Create a Robot: ");
                ui.text_edit_singleline(&mut self.name);
                egui::ComboBox::from_label("Select one!")
                    .selected_text(format!("{:?}", self.bot_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.bot_type, BotType::Unipedal, "Unipedal");
                        ui.selectable_value(&mut self.bot_type, BotType::Bipedal, "Bipedal");
                        ui.selectable_value(&mut self.bot_type, BotType::Quadrupedal, "Quadrupedal");
                    }
                );
            });
            if ui.button("Create Robot").clicked() {
                let bot = Robot::new(self.name.clone(), self.bot_type);
                println!("{:?}", bot);
            }
            // ui.label(format!("Hello '{:?}'", bot));
        });
    }
}