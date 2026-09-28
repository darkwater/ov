use egui::{Color32, Pos2, Tooltip, Vec2};
use rovapi_models::{LocationType, Stop};
use walkers::Plugin;

use crate::app::db::Database;

pub struct StopsPlugin<'a> {
    pub db: &'a Database,
}

impl Plugin for StopsPlugin<'_> {
    fn run(
        self: Box<Self>,
        ui: &mut egui::Ui,
        response: &egui::Response,
        projector: &walkers::Projector,
        map_memory: &walkers::MapMemory,
    ) {
        for stop in self.db.stops.values() {
            let Some(point) = stop.point() else {
                continue;
            };

            let screen_pos = Pos2::ZERO + projector.project(point);

            let radius = 5.;
            let rect = egui::Rect::from_center_size(screen_pos, Vec2::splat(radius * 3.));
            let color = match stop.location_type {
                LocationType::StopOrPlatform => Color32::BLUE,
                LocationType::Station => Color32::RED,
                LocationType::EntranceOrExit => Color32::ORANGE,
                LocationType::GenericNode => Color32::MAGENTA,
                LocationType::BoardingArea => Color32::GREEN,
            };

            if let Some(parent) = stop
                .parent_source_id
                .as_ref()
                .and_then(|id| self.db.stops.get(id))
                && let Some(parent_point) = parent.point()
            {
                let parent_pos = Pos2::ZERO + projector.project(parent_point);

                ui.painter()
                    .line_segment([parent_pos, screen_pos], (2., color));
            } else {
                ui.painter().circle_filled(screen_pos, radius, color);
            }

            if ui.rect_contains_pointer(rect) {
                Tooltip::for_widget(response).at_pointer().show(|ui| {
                    let Stop {
                        source_id,
                        code,
                        name,
                        latitude,
                        longitude,
                        location_type,
                        parent_source_id,
                        platform_code,
                    } = stop;

                    ui.label(format!("ID: {source_id}"));
                    ui.label(format!("Code: {code:?}"));
                    ui.label(format!("Name: {name}"));
                    ui.label(format!("Latitude: {latitude:?}"));
                    ui.label(format!("Longitude: {longitude:?}"));
                    ui.label(format!("Location Type: {location_type:?}"));
                    ui.label(format!("Parent Source ID: {parent_source_id:?}"));
                    ui.label(format!("Platform Code: {platform_code:?}"))
                });
            }
        }
    }
}
