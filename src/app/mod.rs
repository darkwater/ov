mod db;
mod map;
mod message;
mod selection;

use std::sync::mpsc;

use egui::Frame;

use self::{db::Database, map::MapView, message::Message, selection::Selection};

pub struct OvApp {
    map: MapView,
    db: Database,
    messages: mpsc::Receiver<Message>,
    selection: Selection,
}

impl OvApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx
            .all_styles_mut(|s| s.interaction.tooltip_delay = 0.1);

        Self {
            map: MapView::new(cc.egui_ctx.clone()),
            db: Database::default(),
            messages: Message::init(cc.egui_ctx.clone()),
            selection: Selection::default(),
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
                Message::FetchedStopGroups(stop_groups) => {
                    self.db.add_stop_groups(stop_groups);
                }
                Message::FetchedStopGroupDetails(stop_group_details) => {
                    self.db
                        .stop_group_details
                        .insert(stop_group_details.group.id.clone(), stop_group_details);
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
                        String::from("http://192.168.0.106:4941/v1/stop-groups/in-rect")
                            + &format!("?min_lat={min_lat}&min_lon={min_lon}")
                            + &format!("&max_lat={max_lat}&max_lon={max_lon}&limit=25000"),
                    );
                    ehttp::fetch(req, |response| {
                        let stops = response
                            .expect("stop groups error")
                            .json()
                            .expect("stop groups json error");

                        Message::FetchedStopGroups(stops).send();
                    });
                }
            });
        });

        if let Some(details) = self.selection.stop_group_details(&self.db) {
            egui::Panel::right("right panel").show(ui, |ui| {
                ui.heading(&details.group.name);
                ui.label(format!("ID: {}", details.group.id));

                ui.heading("Members");
                for member in &details.members {
                    let stop = &member.location;
                    ui.label(format!("Name: {}", stop.name));
                    ui.label(
                        member
                            .route_ids
                            .iter()
                            .fold(String::from("Routes:"), |acc, route_id| {
                                acc + &format!(" {route_id}")
                            }),
                    );
                    ui.label(
                        member
                            .agency_ids
                            .iter()
                            .fold(String::from("Agencies:"), |acc, route_id| {
                                acc + &format!(" {route_id}")
                            }),
                    );
                }
            });
        }

        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ui, |ui| {
                self.map.show(ui, &self.db, &mut self.selection);
            });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
