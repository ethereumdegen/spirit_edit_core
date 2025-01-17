use crate::utils::StringUtilsExt;
use crate::placement::PlacementEvent;
use crate::prefabs::SpawnPrefabEvent;
use crate::{doodads::PlaceClayTileEvent };
use bevy_clay_tiles::clay_tile_block::ClayTileBlock;
use std::fs;
use bevy::prelude::*;
 

use std::path::Path;

use crate::doodads::PlaceDoodadEvent;

#[derive(Component)]
pub struct ZoneComponent {}

#[derive(Event)]
pub enum ZoneEvent {
   // SetZoneAsPrimary(Entity),
   // SaveZoneToFile(Entity),
    SaveAllZones,
    CreateNewZone(String),
    LoadZoneFile(String),
    //ResetPrimaryZone,
}


#[derive(Event)]
pub struct SaveZoneToFileEvent(pub Entity) ;



pub mod zone_file;

use zone_file::ZoneFileV2 as ZoneFile;

use self::zone_file::ZoneEntityV2 as ZoneEntity;

//use self::zone_file::CustomPropsComponent;

const DEFAULT_FILENAME: &str = "zone01";

#[derive(Default, Component)]
pub struct NotInScene;

/*
#[derive(Default)]
pub struct ZoneWindowState {
    create_filename: String,
    load_filename: String,
    zone_create_result: Option<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
}

pub struct ZoneWindow;

impl EditorWindow for ZoneWindow {
    type State = ZoneWindowState;
    const NAME: &'static str = "Zones";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<ZoneWindow>().unwrap();

        let zone_resource = world.resource::<ZoneResource>();
        let primary_zone_entity = zone_resource.primary_zone;

        let primary_zone_name = primary_zone_entity
            .and_then(|ent| {
                // Temporarily fetch the component to avoid holding the borrow
                world.get::<Name>(ent).map(|n| n.as_str().to_owned())
            })
            .unwrap_or_else(|| "None".to_owned());

      ui.horizontal(|ui| {

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Primary zone: {:?}", primary_zone_name.clone()));
                if ui.button("Reset").clicked() {
                    world.send_event::<ZoneEvent>(ZoneEvent::ResetPrimaryZone);
                }
            });

            //create zone
            ui.horizontal(|ui| {
                let res = egui::TextEdit::singleline(&mut state.create_filename)
                    .hint_text(DEFAULT_FILENAME)
                    .desired_width(120.0)
                    .show(ui);

                if res.response.changed() {
                    state.zone_create_result = None;
                }

               // let enter_pressed = ui.input(|input| input.key_pressed(egui::Key::Enter));

                if ui.button("Create Zone").clicked()   {
                    let create_filename = if state.create_filename.is_empty() {
                        DEFAULT_FILENAME
                    } else {
                        &state.create_filename
                    };
                    let mut query = world.query_filtered::<Entity, Without<NotInScene>>();
                    // let entitys = query.iter(world).collect();
                    state.zone_create_result = Some(create_zone(world, create_filename));
                }
            });

            ui.horizontal(|ui| {
                let res = egui::TextEdit::singleline(&mut state.load_filename)
                    .hint_text(DEFAULT_FILENAME)
                    .desired_width(120.0)
                    .show(ui);

                if res.response.changed() {
                    state.zone_create_result = None;
                }

                let enter_pressed = ui.input(|input| input.key_pressed(egui::Key::Enter));

                if ui.button("Load Zone").clicked()   {
                    let load_filename = if state.load_filename.is_empty() {
                        DEFAULT_FILENAME
                    } else {
                        &state.load_filename
                    };
                    let mut query = world.query_filtered::<Entity, Without<NotInScene>>();
                    // let entitys = query.iter(world).collect();
                    state.zone_create_result = Some(load_zone(world, load_filename));
                }
            })
            // ----- h
        }); // ---- v

           ui.vertical(|ui| {

              ui.label(format!("--- " ) );

                if ui.button("Load All Zones").clicked()   {
                   
                    // let entitys = query.iter(world).collect();
                    state.zone_create_result = Some(load_all_zones(world));
                }


             }); // ---- v

      }); // ---- H


        if let Some(status) = &state.zone_create_result {
            match status {
                Ok(()) => {
                    ui.label(RichText::new("Success!").color(egui::Color32::GREEN));
                }
                Err(error) => {
                    ui.label(RichText::new(error.to_string()).color(egui::Color32::RED));
                }
            }
        }
    }
}
*/

pub fn create_zone(
    //  world: &mut World,
    world: &mut World,
    name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    world.send_event::<ZoneEvent>(ZoneEvent::CreateNewZone(name.into()));

    Ok(())
}

pub fn load_zone(
    world: &mut World,
    name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    world.send_event::<ZoneEvent>(ZoneEvent::LoadZoneFile(name.into()));

    Ok(())
}

pub fn load_all_zones(
    world: &mut World
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {


    let zone_file_names:Vec<String> = get_all_zone_file_names();

    for file_name in zone_file_names {
          world.send_event::<ZoneEvent>(ZoneEvent::LoadZoneFile(file_name.into()));
    }

  

    Ok(())
}


pub fn get_all_zone_file_names() -> Vec<String> {
    let zones_dir = Path::new("assets/zones");

    let file_names = match fs::read_dir(zones_dir) {
        Ok(entries) => entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "ron") {
                        path.file_stem().and_then(|stem| stem.to_str().map(|s| s.to_string()))
                    } else {
                        None
                    }
                })
            })
            .collect(),
        Err(err) => {
            eprintln!("Error reading directory: {:?}", err);
            Vec::new()
        }
    };

    file_names
}

pub fn handle_zone_events(
    mut commands: Commands,
    mut evt_reader: EventReader<ZoneEvent>,

     

    children_query: Query<&Children, With<Name>>,   



    //change me to entity ref ..
    zone_entity_query: Query<
      Entity,
      //(&Name, &Transform, Option<&CustomPropsComponent>, Option<&ClayTileBlock>),
     With<ZoneComponent>>, 

    mut save_zone_evt_writer: EventWriter<SaveZoneToFileEvent>,

    mut placement_evt_writer: EventWriter<PlacementEvent>,

    mut spawn_doodad_event_writer: EventWriter<PlaceDoodadEvent>,

    mut spawn_clay_tile_event_writer: EventWriter<PlaceClayTileEvent>,
    
    mut spawn_prefab_event_writer: EventWriter<SpawnPrefabEvent>,
) {
    for evt in evt_reader.read() {
        match evt {
            ZoneEvent::CreateNewZone(name) => {

                let name_fixed =  name.ensure_ends_with(".zone");

                let created_zone = commands
                    .spawn((Transform::default(),Visibility::default()))
                    .insert(ZoneComponent {})
                    .insert(Name::new( name_fixed .to_string()))
                    .id();

                placement_evt_writer.send(PlacementEvent::SetPlacementParent( Some(created_zone) ));

               // zone_resource.primary_zone = Some(created_zone);
            }

            
            ZoneEvent::SaveAllZones => {
                
                for zone_entity in zone_entity_query.iter(){

                    save_zone_evt_writer.send(
                        SaveZoneToFileEvent(zone_entity)
                    );
                }
                //loop thru each  Zone entity 
              //  and fire off an evt 



            }

           /* ZoneEvent::SaveZoneToFile(ent) => {
                //this is kind of wacky but we are using this as a poor mans name query
                let Some((zone_name_comp, _, _, _)) = zone_entity_query.get(ent.clone()).ok() else {
                    return;
                };

                let zone_name: &str = zone_name_comp.as_str();


                let fixed_zone_name = match zone_name.ends_with( "zone.ron" ) || zone_name.ends_with( "zone" ){

                    true => {

                          let   parts: Vec<&str> = zone_name.split('.').collect();
                         
                            parts.first().unwrap() .to_string()  
                           

                      }, 
                    false => zone_name.to_string()

                };
                

                let mut all_children: Vec<Entity> = Vec::new();

                for child in DescendantIter::new(&children_query, ent.clone()) {
                    all_children.push(child);
                }

                let zone_file = ZoneFile::new(all_children, &zone_entity_query );

                let zone_file_name = format!("assets/zones/{}.zone.ron", fixed_zone_name);

                let ron = ron::ser::to_string(&zone_file).unwrap();
                let file_saved = std::fs::write(zone_file_name, ron);

                println!("exported zone ! {:?}", file_saved);
            }*/

            ZoneEvent::LoadZoneFile(zone_name) => {

                let fixed_zone_name = match zone_name.ends_with( "zone.ron" ) || zone_name.ends_with( "zone" ){

                    true => {

                          let   parts: Vec<&str> = zone_name.split('.').collect();
                         
                            parts.first().unwrap() .to_string()  
                           

                      }, 
                    false => zone_name.to_string()

                };


                let file_name = format!("assets/zonesv2/{}.zone.ron", fixed_zone_name);
 
                 let path = Path::new(&file_name);

                // Read the file into a string
               /* let Ok(file_content) = std::fs::read_to_string(path) else {
                    println!("Could not find file {:?}", file_name);
                    return;
                };


                let zone_file = match ron::from_str::<ZoneFile>(&file_content) {

                    Ok(f) => f ,

                    Err(e) =>  {
                        eprintln!("Could not parse file {:?} {:?}", file_name, e); 
                        return;
                    }
                };*/

                let Some(zone_file) = ZoneFile::load_from_path(  path  ) else {
                    eprintln!("could not parse zone file {:?}", path);
                    return;
                };
               
                //spawnn the zone entity and set it as primary

                let created_zone = commands
                    .spawn((Transform::default(),Visibility::default()))
                    .insert(ZoneComponent {})
                    .insert(Name::new(zone_name.to_string()))
                    
                    .id();


                placement_evt_writer.send(PlacementEvent::SetPlacementParent( Some(created_zone) ));


                 

                //trigger spawn doodad events

                for zone_entity in zone_file.entities {

                    match zone_entity {

                        ZoneEntity::Doodad { ref name, ref transform, ref custom_props } => {

                             spawn_doodad_event_writer.send({
                                PlaceDoodadEvent {
                                    doodad_name: name.clone(),
                                    position: zone_entity.get_position(),
                                    rotation_euler: Some(zone_entity.get_rotation_euler()),
                                    scale: Some(zone_entity.get_scale()),
                                    custom_props: zone_entity.get_custom_props().clone(),
                                  //  clay_tile_block_data: zone_entity.clay_tile_block_data.clone(), 
                                    force_parent:Some(created_zone),
                                    auto_select: false 
                                }
                            });


                        },
                        ZoneEntity::ClayTile { ref transform,   ref clay_tile_block } => {

                              spawn_clay_tile_event_writer.send({
                                PlaceClayTileEvent {
                                    //doodad_name: name.clone(),
                                    position: zone_entity.get_position(),
                                    rotation_euler: Some(zone_entity.get_rotation_euler()),
                                    scale: Some(zone_entity.get_scale()),
                                   // custom_props: zone_entity.get_custom_props().clone(),
                                    clay_tile_block_data: clay_tile_block.clone(), 
                                    zone:Some(created_zone)
                                }
                            });


                        },
                        ZoneEntity::Prefab { ref name, ref transform } => {

                            spawn_prefab_event_writer.send(

                                SpawnPrefabEvent {
                                 position:zone_entity.get_position(),
                                 rotation_euler: Some(zone_entity.get_rotation_euler()),
                                  prefab_name: name.clone(), 
                                  zone: Some(created_zone)
                               }
                            );



                        }

                    }


                   



                }
            }
        }
    }
}



pub fn handle_save_zone_events(
  //  mut commands: Commands,
    mut evt_reader: EventReader<SaveZoneToFileEvent>,

    entity_ref_query: Query<EntityRef>  ,

      
) {
    for evt in evt_reader.read() {

        let ent = evt.0;

        let Some(zone_entity_ref) = entity_ref_query.get(ent).ok() else {continue};
          
        //this is kind of wacky but we are using this as a poor mans name query
        let Some(zone_name_comp) = zone_entity_ref.get::<Name>() else {
            return;
        };

        let zone_name: &str = zone_name_comp.as_str();

        println!("handle save zone.." );


        let fixed_zone_name = match zone_name.ends_with( "zone.ron" ) || zone_name.ends_with( "zone" ){

            true => {

                  let   parts: Vec<&str> = zone_name.split('.').collect();
                 
                    parts.first().unwrap() .to_string()  
                   

              }, 
            false => zone_name.to_string()

        };
        

       // let mut all_children: Vec<Entity> = Vec::new();

        /*for child in DescendantIter::new(&children_query, ent.clone()) {
            all_children.push(child);
        }*/

        let mut zone_entities:Vec<ZoneEntity> = Vec::new();

       /* let  Some(zone_children)  = zone_entity_ref.get::<Children>()  else {

            warn!( "cannot save an empty zone.."  );
            continue
        };*/

        let   zone_children   = zone_entity_ref.get::<Children>()  ;

        let zone_children = zone_children.map(|c| c.into_iter().collect::<Vec<_>>() ).unwrap_or_default(); 

        for child_entity in zone_children {


            if let Some(child_entity_ref) = entity_ref_query.get( *child_entity ).ok() {

                if let Some(zone_entity) = ZoneEntity::from_entity_ref( &child_entity_ref ) {
                    zone_entities.push(zone_entity);
                }
            }


        }




        let zone_file = ZoneFile {

            entities: zone_entities,
            ..default()
        };

        let zone_file_name = format!("assets/zonesv2/{}.zone.ron", fixed_zone_name);

        let ron = ron::ser::to_string(&zone_file).unwrap();
        let file_saved = std::fs::write(zone_file_name, ron);

        println!("exported zone ! {:?}", file_saved);
            
    }
}
