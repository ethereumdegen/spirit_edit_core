
use bevy::prelude::*;

pub mod prefab_file;

#[derive(Component)]
pub struct PrefabComponent ;





#[derive(Event)]
pub struct SavePrefabToFileEvent(pub Entity) ;


#[derive(Event)]
pub struct SpawnPrefabEvent {

	pub position: Vec3,
    
    pub rotation_euler: Option<Vec3>, 

    pub prefab_name: String,
    //pub custom_props: Option<CustomPropsMap>,


    pub zone: Option<Entity> ,



} 