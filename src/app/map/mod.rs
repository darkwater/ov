use egui::Context;
use walkers::{HttpTiles, Map, MapMemory, lat_lon, sources::OpenStreetMap};

pub struct MapView {
    tiles: HttpTiles,
    map_memory: MapMemory,
}

impl MapView {
    pub fn new(egui_ctx: Context) -> Self {
        Self {
            tiles: HttpTiles::new(OpenStreetMap, egui_ctx),
            map_memory: MapMemory::default(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.add(
            Map::new(
                Some(&mut self.tiles),
                &mut self.map_memory,
                lat_lon(53.21881846, 6.56784254),
            )
            .pull_to_my_position_threshold(10.)
            .zoom_with_ctrl(false),
        );
    }
}
