 
 
use bevy::prelude::*;

use doodads::DoodadPlugin;
use zones::{zone_file::{CustomProp, CustomPropsComponent},  ZoneEvent,SaveZoneToFileEvent, ZoneResource};

pub use bevy_clay_tiles; // export

pub mod doodads;
pub mod zones;
pub mod placement;
pub mod prefabs;


pub struct SpiritEditCorePlugin {}
impl Plugin for SpiritEditCorePlugin {
    fn build(&self, app: &mut App) {
        //put this inside of zone plugin ?
         app
           	

             .add_plugins(DoodadPlugin {})
            

           	 .add_event::<placement::PlacementEvent>()
           	
            .init_resource::<placement::PlacementResource>()

            .init_resource::<placement::PlacementToolsState>()

             .add_event::<prefabs::SpawnPrefabEvent>()
            .add_event::<ZoneEvent>()
            .add_event::<SaveZoneToFileEvent>()
           
            .register_type::<CustomPropsComponent>() //reflect
              .register_type::<CustomProp>() //reflect
           
            .init_resource::<ZoneResource>() 

           
         
            
            .add_systems(Update, (
                zones::handle_zone_events,
                zones::handle_save_zone_events
            ).chain())


             
             


            ;
    }
}


