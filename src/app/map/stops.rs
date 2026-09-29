use egui::{Color32, Pos2, Tooltip, Vec2};
use rovapi_models::{LocationType, Stop, StopGroup};
use walkers::Plugin;

use crate::app::{db::Database, message::Message, selection::Selection};

pub struct StopsPlugin<'a> {
    pub db: &'a Database,
    pub selection: &'a mut Selection,
}

impl Plugin for StopsPlugin<'_> {
    fn run(
        self: Box<Self>,
        ui: &mut egui::Ui,
        response: &egui::Response,
        projector: &walkers::Projector,
        map_memory: &walkers::MapMemory,
    ) {
        for stop in self.db.stop_groups.values() {
            let Some(point) = stop.point() else {
                continue;
            };

            let screen_pos = Pos2::ZERO + projector.project(point);

            let radius = 5.;
            let rect = egui::Rect::from_center_size(screen_pos, Vec2::splat(radius * 3.));
            // let color = match stop.location_type {
            //     LocationType::StopOrPlatform => Color32::BLUE,
            //     LocationType::Station => Color32::RED,
            //     LocationType::EntranceOrExit => Color32::ORANGE,
            //     LocationType::GenericNode => Color32::MAGENTA,
            //     LocationType::BoardingArea => Color32::GREEN,
            // };
            let color = Color32::from_rgb(0, 50, 255);

            // if let Some(parent) = stop
            //     .parent_source_id
            //     .as_ref()
            //     .and_then(|id| self.db.stops.get(id))
            //     && let Some(parent_point) = parent.point()
            // {
            //     let parent_pos = Pos2::ZERO + projector.project(parent_point);
            //
            //     ui.painter()
            //         .line_segment([parent_pos, screen_pos], (2., color));
            // } else {
            ui.painter().circle_filled(screen_pos, radius, color);
            // }

            if ui.rect_contains_pointer(rect) {
                Tooltip::for_widget(response).at_pointer().show(|ui| {
                    let StopGroup {
                        id,
                        name,
                        latitude,
                        longitude,
                        bounds,
                        member_count,
                    } = stop;

                    ui.label(format!("ID: {id}"));
                    ui.label(format!("Name: {name}"));
                    ui.label(format!("Latitude: {latitude:?}"));
                    ui.label(format!("Longitude: {longitude:?}"));
                    ui.label(format!("Bounds: {bounds:?}"));
                    ui.label(format!("Member Count: {member_count:?}"));
                });

                if response.clicked() {
                    self.selection.stop_group_id = Some(stop.id.clone());

                    let req = ehttp::Request::get(
                        String::from("http://192.168.0.106:4941/v1/stop-groups/")
                            + &stop.id.to_string(),
                    );
                    ehttp::fetch(req, |response| {
                        let stops = response
                            .expect("stop groups error")
                            .json()
                            .expect("stop groups json error");

                        Message::FetchedStopGroupDetails(stops).send();
                    });
                }
            }
        }
    }
}
