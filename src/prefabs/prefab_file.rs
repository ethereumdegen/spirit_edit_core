

use serde::Serialize;
use serde::Deserialize;
use crate::zones::zone_file::ZoneEntity;
use bevy::prelude::*;

#[derive(Serialize, Deserialize,Default)]
pub struct PrefabFile {
    pub translation_offset: Option<Vec3>,  //usually none 
    pub entities: Vec<ZoneEntity>, 
}
