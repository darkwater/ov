mod db;
mod map;
mod message;

use std::sync::mpsc;

use self::{db::Database, map::MapView, message::Message};

pub struct OvApp {
    map: MapView,
    db: Database,
    messages: mpsc::Receiver<Message>,
}

impl OvApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx
            .all_styles_mut(|s| s.interaction.tooltip_delay = 0.1);

        Self {
            map: MapView::new(cc.egui_ctx.clone()),
            db: Database::default(),
            messages: Message::init(cc.egui_ctx.clone()),
        }
    }
}

impl eframe::App for OvApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(message) = self.messages.try_recv() {
            match message {
                Message::FetchedStops(stops) => {
                    self.db.add_stops(stops);
                }
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Fetch Stops").clicked() {
                    let min_lat = self.map.viewport_rect().min().y;
                    let min_lon = self.map.viewport_rect().min().x;
                    let max_lat = self.map.viewport_rect().max().y;
                    let max_lon = self.map.viewport_rect().max().x;

                    let req = ehttp::Request::get(
                        String::from("http://192.168.0.106:4941/v1/stops/in-rect")
                            + &format!("?min_lat={min_lat}&min_lon={min_lon}")
                            + &format!("&max_lat={max_lat}&max_lon={max_lon}&limit=25000"),
                    );
                    ehttp::fetch(req, |response| {
                        let stops = response
                            .expect("stops error")
                            .json()
                            .expect("stops json error");

                        Message::FetchedStops(stops).send();
                    });
                }
            });
        });

        egui::Panel::right("right panel").show(ui, |ui| {
            ui.heading("Stops");
            for stop in self.db.stops.values() {
                ui.label(format!("{} ({})", stop.name, stop.source_id));
            }
        });

        egui::CentralPanel::default().show(ui, |ui| {
            self.map.show(ui, &self.db);
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
