

use bevy::prelude::*;


use bevy::utils::HashMap;

use super::prefab_file::PrefabFile;



pub type PrefabName = String;

pub type PrefabDefinition = PrefabFile; 

#[derive(Resource, Default)]
pub struct PrefabDefinitionsResource {
    pub loaded_prefab_definitions: Option< HashMap<PrefabName,PrefabDefinition> >,
 
}

impl PrefabDefinitionsResource {

    pub fn get_prefab_definition_by_name(&self, prefab_name: &String) -> Option<&PrefabDefinition>{

        return self.loaded_prefab_definitions.as_ref().map(|d| d.get(prefab_name)).flatten()
    }
}


