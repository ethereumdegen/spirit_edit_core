

use serde::Serialize;
use serde::Deserialize;
use crate::zones::zone_file::ZoneEntityV2;
use bevy::prelude::*;

#[derive(Serialize, Deserialize,Default)]
pub struct PrefabFile {
    pub translation_offset: Option<Vec3>,  //usually none 
    pub entities: Vec<ZoneEntityV2>, 
}
