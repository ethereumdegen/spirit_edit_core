use crate::placement::PlacementToolsState;
//use bevy_editor_pls_core::EditorEvent;
//use bevy_editor_pls_core::Editor;
use bevy_clay_tiles::clay_tile_block::ClayTileBlock;
//use crate::doodads::doodad_placement_preview::DoodadPlacementPlugin;
use bevy::{asset::ReflectAsset, reflect::TypeRegistry};


use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_mod_raycast::immediate::RaycastSettings;
use rand::Rng;

use bevy::utils::HashMap;


use crate::doodads::doodad_manifest::RenderableType;
 
use crate::zones::zone_file::{CustomPropsComponent,CustomPropsMap};
 

/*
use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_editor_pls_core::{Editor, EditorEvent};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self, ScrollArea};*/

use bevy_common_assets::ron::RonAssetPlugin;

use bevy_mod_raycast::cursor::CursorRay;

use bevy_mod_raycast::prelude::Raycast;

use self::doodad::{DoodadComponent,    };
use self::doodad_manifest::{DoodadDefinition, DoodadManifest, DoodadDefinitionsResource, DoodadTagMapResource};
//use self::doodad_placement_preview::DoodadPlacementComponent;

 
//pub mod material_overrides; 
pub mod doodad_manifest;
pub mod picking;
pub mod doodad;
pub mod rotate;
//pub mod doodad_placement_preview;


pub struct DoodadPlugin {}
impl Plugin for DoodadPlugin {
    fn build(&self, app: &mut App) {
        //put this inside of zone plugin ?
         app
             .add_event::< picking::SelectDoodadEvent>()
             .add_event::< PlaceDoodadEvent>()
              .add_event::< PlaceClayTileEvent>()
             .add_event::< DoodadToolEvent>()

             .init_resource::<DoodadToolState>()

             .init_resource::<DoodadTagMapResource>()
             .init_resource::<DoodadDefinitionsResource>()


              .add_systems(Update, rotate::handle_rotate_by_degrees)

          //    .add_plugins(DoodadPlacementPlugin {} )
            
        //     .add_systems(Update, picking::update_picking_doodads)
           

            ;
    }
}


 




#[derive(Resource, Default)]
pub struct DoodadToolState {
    pub selected: Option<String>,
}



#[derive(Event)]
pub enum DoodadToolEvent {
    SetSelectedDoodad(Option<String>)
}

#[derive(Component)]
pub struct DoodadProto;


#[derive(Component)]
pub struct DoodadNeedsModelAttached;



#[derive(Event)]
pub struct PlaceDoodadEvent {
    pub position: Vec3,
    pub scale: Option<Vec3>,
    pub rotation_euler: Option<Vec3>,
    pub doodad_name: String,
    pub custom_props: Option<CustomPropsMap>,
    pub zone: Option<Entity> ,

  //  pub clay_tile_block_data: Option<ClayTileBlock >, //dont love this but its K 
    // pub doodad_definition: DoodadDefinition
}



#[derive(Event)]
pub struct PlaceClayTileEvent {
    pub position: Vec3,
    pub scale: Option<Vec3>,
    pub rotation_euler: Option<Vec3>,
  //  pub doodad_name: String,
   // pub custom_props: Option<CustomPropsMap>,
    pub zone: Option<Entity> ,

    pub clay_tile_block_data: ClayTileBlock , //dont love this but its K 
    // pub doodad_definition: DoodadDefinition
}

// --------------------------------------------------------

