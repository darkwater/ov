mod map;

use self::map::MapView;

pub struct OvApp {
    map: MapView,
}

impl OvApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            map: MapView::new(cc.egui_ctx.clone()),
        }
    }
}

impl eframe::App for OvApp {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.map.show(ui);
        });
    }
}
