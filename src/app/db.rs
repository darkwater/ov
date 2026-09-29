use std::collections::btree_map::BTreeMap;

use rovapi_models::{Stop, StopGroup, StopGroupDetails, StopGroupId, StopId};

#[derive(Default)]
pub struct Database {
    pub stops: BTreeMap<StopId, Stop>,
    pub stop_groups: BTreeMap<StopGroupId, StopGroup>,
    pub stop_group_details: BTreeMap<StopGroupId, StopGroupDetails>,
}

impl Database {
    pub fn add_stops(&mut self, stops: Vec<Stop>) {
        for stop in stops {
            self.stops.insert(stop.source_id.clone(), stop);
        }
    }

    pub fn add_stop_groups(&mut self, stop_groups: Vec<StopGroup>) {
        for stop_group in stop_groups {
            self.stop_groups.insert(stop_group.id.clone(), stop_group);
        }
    }
}
