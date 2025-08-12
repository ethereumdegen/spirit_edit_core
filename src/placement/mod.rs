use std::time::Duration;

use bevy::prelude::*;
/*use bevy_editor_pls_core::{editor_window::{EditorWindow, EditorWindowContext}, Editor};
use bevy_inspector_egui::egui::{self, RichText};*/

//use bevy_editor_pls_core::{editor_window::{EditorWindow, EditorWindowContext}, Editor};


 

#[derive(Resource)]
pub struct PlacementResource {

    pub grid_lock_delay_timer: Timer ,

    pub placement_parent: Option<Entity>,   //replaces 'primary zone ' 
    
}


 

impl Default for PlacementResource {
    fn default() -> Self {
        PlacementResource {
            // Initialize the Timer with some default value, for example 0.5 seconds
            grid_lock_delay_timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
            placement_parent : None 
        }
    }
}

#[derive(Event)]
pub enum PlacementEvent {

    CloneSelectedDoodad,
    GridLockSelectedDoodad(Vec3),

    SetPlacementParent( Option<Entity> )

} 


#[derive(Resource )]
pub struct PlacementToolsState{
	pub  randomize_yaw: bool,
	pub random_scale_multiplier : f32,
    
    
}


impl Default for PlacementToolsState {

	fn default() -> Self {
	    Self {
	    	randomize_yaw: false,
	    	random_scale_multiplier: 0.0 
	    }
	}

}



pub fn handle_placement_events(
    //mut commands: Commands,
    mut evt_reader: EventReader<PlacementEvent>,

    mut placement_resource: ResMut<PlacementResource>,

   // children_query: Query<&Children, With<Name>>,   



    //change me to entity ref ..
   /* zone_entity_query: Query<
      Entity,
      //(&Name, &Transform, Option<&CustomPropsComponent>, Option<&ClayTileBlock>),
     With<ZoneComponent>>, */

   // mut save_zone_evt_writer: EventWriter<SaveZoneToFileEvent>,

    //mut spawn_doodad_event_writer: EventWriter<PlaceDoodadEvent>,

  //  mut spawn_clay_tile_event_writer: EventWriter<PlaceClayTileEvent>,
    
   // mut spawn_prefab_event_writer: EventWriter<SpawnPrefabEvent>,
) {
    for evt in evt_reader.read() {
        match evt {
            
            PlacementEvent::SetPlacementParent(ent) => {
                placement_resource.placement_parent =  ent.clone();
            }

            _ => {} 
            
        }
    }
}