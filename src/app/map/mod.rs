mod stops;

use egui::{Context, Vec2};
use geo_types::{Coord, Rect};
use walkers::{HttpTiles, Map, MapMemory, lat_lon, sources::OpenStreetMap};

use self::stops::StopsPlugin;
use crate::app::{db::Database, selection::Selection};

pub struct MapView {
    pub tiles: HttpTiles,
    pub memory: MapMemory,
    pub last_rect: Rect,
}

impl MapView {
    pub fn new(egui_ctx: Context) -> Self {
        Self {
            tiles: HttpTiles::new(OpenStreetMap, egui_ctx),
            memory: MapMemory::default(),
            last_rect: Rect::new(Coord::zero(), Coord::zero()),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, db: &Database, selection: &mut Selection) {
        Map::new(Some(&mut self.tiles), &mut self.memory, lat_lon(53.21881846, 6.56784254))
            .with_plugin(StopsPlugin { db, selection })
            .zoom_with_ctrl(false)
            .show(ui, |_ui, response, projector, _map_memory| {
                let min = projector.unproject(Vec2::ZERO);
                let max = projector.unproject(response.rect.size());
                self.last_rect = Rect::new(min, max);
            });
    }

    pub fn viewport_rect(&self) -> Rect {
        self.last_rect
    }
}
