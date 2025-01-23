 
 
use bevy::prelude::*;

 
use zones::{zone_file::{CustomProp, CustomPropsComponent},  ZoneEvent,SaveZoneToFileEvent };

pub use bevy_clay_tiles; // export

pub mod doodads;
pub mod zones;
pub mod placement;
pub mod prefabs;


mod utils;

pub mod gltf_models;

pub struct SpiritEditCorePlugin {}
impl Plugin for SpiritEditCorePlugin {
    fn build(&self, app: &mut App) {
        //put this inside of zone plugin ?
         app
           	

             .add_plugins(doodads::doodad_plugin)
             .add_plugins(prefabs::prefab_plugin)
             .add_plugins(gltf_models::gltf_models_plugin   )

           	 .add_event::<placement::PlacementEvent>()
           	
            .init_resource::<placement::PlacementResource>()

            .init_resource::<placement::PlacementToolsState>()

             .add_event::<prefabs::PrefabEvent>()
             .add_event::<prefabs::SpawnPrefabEvent>()
            .add_event::<ZoneEvent>()
            .add_event::<SaveZoneToFileEvent>()
            .add_event::<prefabs::SavePrefabToFileEvent>()
           
            .register_type::<CustomPropsComponent>() //reflect
            .register_type::<CustomProp>() //reflect
           
           
           
         
            
            .add_systems(Update, (
                zones::handle_zone_events,
                zones::handle_save_zone_events,
                prefabs::handle_prefab_events ,
                prefabs::handle_save_prefab_events, 
                placement::handle_placement_events,
            ).chain())


             
             


            ;
    }
}


