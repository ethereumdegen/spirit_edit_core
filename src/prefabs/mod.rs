
use crate::{prefabs::prefab_file::PrefabFile, zones::zone_file::ZoneEntityV2 as ZoneEntity};
use bevy::prelude::*;

pub mod prefab_file;
pub mod prefab_definitions;

#[derive(Component)]
pub struct PrefabComponent ;
 



pub fn prefab_plugin(  app: &mut App ){

    app 
    .init_resource::<PrefabToolState>()


    ;
}


#[derive(Resource, Default)]
pub struct PrefabToolState {
    pub selected: Option<String>,
}



#[derive(Event)]
pub enum PrefabToolEvent {
    SetSelectedPrefab(Option<String>)
}



#[derive(Event)]
pub enum PrefabEvent {
   
    SaveAllPrefabs,
 //   CreateNewZone(String),
 //   LoadZoneFile(String),
   
}






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

pub fn handle_prefab_events(  


    mut prefab_evt_reader: EventReader<PrefabEvent>,


    prefab_entity_query: Query<
      Entity,
         
         With<PrefabComponent>>, 

    mut commands : Commands , 

) {

    for evt in prefab_evt_reader.read ()  {





        match evt {


            PrefabEvent::SaveAllPrefabs => {


                for prefab_entity in prefab_entity_query.iter(){


                    commands.send_event( SavePrefabToFileEvent ( prefab_entity.clone() ) );

                    
                }



            }


        }





    }




}




pub fn handle_save_prefab_events(
  //  mut commands: Commands,
    mut evt_reader: EventReader<SavePrefabToFileEvent>,

    entity_ref_query: Query<EntityRef>  ,

      
) {
    for evt in evt_reader.read() {

        let ent = evt.0;

        let Some(prefab_entity_ref) = entity_ref_query.get(ent).ok() else {continue};
          
        //this is kind of wacky but we are using this as a poor mans name query
        let Some(prefab_name_comp) = prefab_entity_ref.get::<Name>() else {
            return;
        };

        let prefab_name: &str = prefab_name_comp.as_str();


        let fixed_prefab_name = match prefab_name.ends_with( "prefab.ron" ) || prefab_name.ends_with( "prefab" ){

            true => {

                  let   parts: Vec<&str> = prefab_name.split('.').collect();
                 
                    parts.first().unwrap() .to_string()  
                   

              }, 
            false => prefab_name.to_string()

        };
        

       // let mut all_children: Vec<Entity> = Vec::new();

        /*for child in DescendantIter::new(&children_query, ent.clone()) {
            all_children.push(child);
        }*/

        let mut zone_entities:Vec<ZoneEntity> = Vec::new();

        let Some(zone_children) = prefab_entity_ref.get::<Children>()  else {continue};

        for child_entity in zone_children {


            if let Some(child_entity_ref) = entity_ref_query.get( *child_entity ).ok() {

                if let Some(zone_entity) = ZoneEntity::from_entity_ref( &child_entity_ref ) {
                    zone_entities.push(zone_entity);
                }
            }


        }




        let prefab_file = PrefabFile {

            entities: zone_entities,

            ..default()
        };

        let zone_file_name = format!("assets/prefabs/{}.prefab.ron", fixed_prefab_name);

        let ron = ron::ser::to_string(&prefab_file).unwrap();
        let file_saved = std::fs::write(zone_file_name, ron);

        println!("exported prefab ! {:?}", file_saved);
            
    }
}

