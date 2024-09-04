

use std::path::Path;
use serde::Serialize;
use serde::Deserialize;
use crate::zones::zone_file::ZoneEntityV2;
use bevy::prelude::*;

#[derive(Serialize, Deserialize,Default)]
pub struct PrefabFile {
    pub translation_offset: Option<Vec3>,  //usually none 
    pub entities: Vec<ZoneEntityV2>, 
}


impl PrefabFile {

    pub fn load_from_path(path: &Path) -> Option<Self> {


 

         let Ok(file_content) = std::fs::read_to_string(path) else {
                    eprintln!("Could not find file {:?}", path);
                    return None;
                };


        let zone_file = match ron::from_str::<Self>(&file_content) {

            Ok(f) => f ,

            Err(e) =>  {
                eprintln!("Could not parse file {:?} {:?}", path, e); 
                return None;
            }
        };

        Some(zone_file)

    


    }

}