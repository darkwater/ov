use std::collections::btree_map::BTreeMap;

use rovapi_models::{Stop, StopId};

#[derive(Default)]
pub struct Database {
    pub stops: BTreeMap<StopId, Stop>,
}

impl Database {
    pub fn add_stops(&mut self, stops: Vec<Stop>) {
        for stop in stops {
            self.stops.insert(stop.source_id.clone(), stop);
        }
    }
}
