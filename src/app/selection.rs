use rovapi_models::{StopGroup, StopGroupDetails, StopGroupId};

use crate::app::db::Database;

#[derive(Default)]
pub struct Selection {
    pub stop_group_id: Option<StopGroupId>,
}

impl Selection {
    pub fn stop_group<'a>(&self, db: &'a Database) -> Option<&'a StopGroup> {
        let id = self.stop_group_id.as_ref()?;
        db.stop_groups.get(id)
    }

    pub fn stop_group_details<'a>(&self, db: &'a Database) -> Option<&'a StopGroupDetails> {
        let id = self.stop_group_id.as_ref()?;
        db.stop_group_details.get(id)
    }
}
