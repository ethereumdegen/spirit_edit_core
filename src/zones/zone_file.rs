use std::f32::consts::PI;
use std::path::Path;

use bevy::{prelude::* };

use bevy::platform::collections::hash_map::HashMap;


use serde::{Deserialize, Serialize};

use bevy_clay_tiles::{ clay_tile_block:: ClayTileBlock };

use crate::prefabs::PrefabComponent; 


#[derive(Serialize, Deserialize,Default,Clone,Debug)]
pub struct ZoneFile {
    pub translation_offset: Option<Vec3>, 
    pub entities: Vec<ZoneEntity>, 
}  

impl ZoneFile {

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


#[derive(Serialize, Deserialize,Default,Clone,Debug)]
pub struct ZoneFileV2 {
    pub translation_offset: Option<Vec3>, 
    pub entities: Vec<ZoneEntityV2>, 
}

impl ZoneFileV2 {

   pub fn from_zone_file(zone_file: ZoneFile) -> Self {
        ZoneFileV2 {
            translation_offset: zone_file.translation_offset,
            entities: zone_file.entities.into_iter().map(ZoneEntityV2::from_zone_entity).collect(),
        }
    }

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



/*
impl ZoneFile {
    pub fn new(
        entities: Vec<Entity>,
        zone_entity_query: &Query<(&Name, &Transform, Option<&CustomPropsComponent>, Option<&ClayTileBlock>)>,
    ) -> Self {
        let mut zone_entities = Vec::new();

        for entity in entities {
            if let Some(zone_entity) = ZoneEntity::from_entity(entity,  zone_entity_query) {
                zone_entities.push(zone_entity);
            }
        }

        Self {
            entities: zone_entities,
        }
    }
}*/

//reflect makes this show up in the inspector
#[derive(Component, Reflect,Default)]
#[reflect(Component)]
pub struct CustomPropsComponent {
    pub props: CustomPropsMap,
}

impl CustomPropsComponent {
    pub fn set_custom_props_if_empty(&mut self, new_props: &CustomPropsMap){

        for (key,val) in new_props.iter(){

            if self.props.get( key ).is_none(){
                  self.props.insert(key.to_string(), val.clone());
            }
          
        }

    }


    pub fn set_custom_props(&mut self, new_props: &CustomPropsMap){

        for (key,val) in new_props.iter(){


            self.props.insert(key.to_string(), val.clone());
        }

    }


     pub fn duplicated(& self ) -> Self {


        let duplicated_keys = vec!["material_override"];

        let mut  new_props = HashMap::new();

        for (key,val) in self.props.iter(){

            if duplicated_keys.contains( & key .as_str() ){


                new_props.insert(key.to_string(), val.clone()) ;
            }
        }


        Self { props: new_props }

    }
}

pub type CustomPropsMap = HashMap<String, CustomProp>;

#[derive(Serialize, Deserialize, Clone, Debug, Reflect, Default)]
#[reflect(Serialize, Deserialize,Default)]  //need to reflect default or else cant use + in editor 
pub enum CustomProp {
    Vec3(Vec3),
    String(String),
    //StringSpecial(StringSpecial),
    Float(f32),
    Integer(i32), 
    Boolean(bool),
    #[default]
    EmptyProp  
}


impl std::fmt::Display for CustomProp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CustomProp::Vec3(vec) => write!(f, "{}", vec),
            CustomProp::String(s) => write!(f, "{}", s),
            CustomProp::Float(num) => write!(f, "{}", num),
            CustomProp::Integer(num) => write!(f, "{}", num),
            CustomProp::Boolean(b) => write!(f, "{}", b),
            CustomProp::EmptyProp => write!(f, ""),
        }
    }
}




//pub struct StringSpecial; 

#[derive(Serialize, Deserialize,Component,Clone,Debug)]
 pub enum ZoneEntityV2 {
    Doodad {


          name: String,

          transform: TransformSimple,

          custom_props: Option<CustomPropsMap>,

    },
    ClayTile  {

          //name: String,

          transform: TransformSimple,

          clay_tile_block: ClayTileBlock,
    } ,
    Prefab{


          name: String,

          transform: TransformSimple,

    }

 }


impl ZoneEntityV2{

    pub fn from_zone_entity(zone_entity: ZoneEntity) -> Self{

        if let Some(clay_tile_block_data) = zone_entity.clay_tile_block_data {

            return Self::ClayTile { 
                transform : zone_entity.transform.clone(),
                 clay_tile_block: clay_tile_block_data.clone() 
             }
        }else {


            return Self::Doodad { 
                name: zone_entity.name.clone(), 
                transform:  zone_entity.transform.clone(),
               custom_props: zone_entity.custom_props
            .clone()
             }
        }

    }



    pub fn get_transform_simple(&self) -> &TransformSimple {
        match self {
            Self::Doodad  { transform, .. } => transform,
            Self::ClayTile  { transform, .. } => transform,
            Self::Prefab  { transform, .. } => transform,
        }
    }

    pub fn get_position(&self) -> Vec3 {
        self.get_transform_simple().translation
    }

    pub fn get_rotation_euler(&self) -> Vec3 {
        self.get_transform_simple().rotation
    }

    pub fn get_scale(&self) -> Vec3 {
        self.get_transform_simple().scale
    }

    pub fn get_custom_props(&self) -> &Option<CustomPropsMap> {
        match self {
            Self::Doodad  { custom_props, .. } => custom_props,
             
            _ => &None,
        }
    }

    pub fn from_entity_ref(
        entity_ref: &EntityRef 
        ) -> Option<Self> {

        let Some(name_comp) = entity_ref.get::<Name>() else {return None};
        let Some(xform) = entity_ref.get::<Transform>() else {return None};
        let custom_props_component = entity_ref.get::<CustomPropsComponent>() ;
      

        let clay_tile_block_data = entity_ref.get::<ClayTileBlock>() ;
        let prefab_component = entity_ref.get::<PrefabComponent>();

        if let Some( clay_tile_block_data ) = clay_tile_block_data {

            return Some(  
                ZoneEntityV2::ClayTile { transform: xform.clone().into(), clay_tile_block: clay_tile_block_data.clone() }
            )

        }



        if prefab_component.is_some(){

            return Some(  
                ZoneEntityV2::Prefab { 
                 name: name_comp.as_str().to_string(), 
                 transform: xform.clone().into()
             }
            )

        }



      //  if let Some((name, xform, custom_props_component, clay_tile_block_data)) = zone_entity_query.get(entity).ok() {
            let custom_props = custom_props_component.and_then(|comp| Some(comp.props.clone()));

             return Some(  
                ZoneEntityV2::Doodad { 
                 name: name_comp.as_str().to_string(),
                transform: xform.clone().into(),
                custom_props,
                 }
            );


            /*return Some(Self {
                name: name_comp.as_str().to_string(),
                transform: xform.clone().into(),
                custom_props,
                clay_tile_block_data: clay_tile_block_data.cloned()
            });*/
     //   }

      //  None

    } 

}



#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct ZoneEntity {
    pub name: String,

    pub transform: TransformSimple,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_props: Option<CustomPropsMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clay_tile_block_data: Option<ClayTileBlock>,

}


 /*




impl ZoneEntity {
    pub fn get_position(&self) -> Vec3 {
        self.transform.translation
    }

    pub fn get_rotation_euler(&self) -> Vec3 {
        self.transform.rotation
    }

    pub fn get_scale(&self) -> Vec3 {
        self.transform.scale
    }

    pub fn get_custom_props(&self) -> &Option<CustomPropsMap> {
        &self.custom_props
    }

    pub fn from_entity_ref(
        entity_ref: &EntityRef 
        ) -> Option<Self> {

        let Some(name_comp) = entity_ref.get::<Name>() else {return None};
        let Some(xform) = entity_ref.get::<Transform>() else {return None};
        let custom_props_component = entity_ref.get::<CustomPropsComponent>() ;
        let clay_tile_block_data = entity_ref.get::<ClayTileBlock>() ;

      //  if let Some((name, xform, custom_props_component, clay_tile_block_data)) = zone_entity_query.get(entity).ok() {
            let custom_props = custom_props_component.and_then(|comp| Some(comp.props.clone()));

            return Some(Self {
                name: name_comp.as_str().to_string(),
                transform: xform.clone().into(),
                custom_props,
                clay_tile_block_data: clay_tile_block_data.cloned()
            });
     //   }

      //  None

    }

    //clean this up and move to root .. 
    /*fn from_entity(
        entity: Entity,
        zone_entity_query: &Query<(&Name, &Transform, Option<&CustomPropsComponent>, Option<&ClayTileBlock>)>,
    ) -> Option<Self> {
        if let Some((name, xform, custom_props_component, clay_tile_block_data)) = zone_entity_query.get(entity).ok() {
            let custom_props = custom_props_component.and_then(|comp| Some(comp.props.clone()));

            return Some(Self {
                name: name.as_str().to_string(),
                transform: xform.clone().into(),
                custom_props,
                clay_tile_block_data: clay_tile_block_data.cloned()
            });
        }

        None
    }*/
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformSimple {
    pub translation: Vec3,
    pub rotation: Vec3, //euler
    pub scale: Vec3,
}


impl Default for TransformSimple {
    fn default() -> Self {
        Self {
            translation: (0.0, 0.0, 0.0).into(),
            rotation: (0.0, 0.0, 0.0).into(),
            scale: (1.0, 1.0, 1.0).into(),
        }
    }
}



impl From<Transform> for TransformSimple {
    fn from(transform: Transform) -> Self {
        // Extract translation directly
        let translation = transform.translation;

        // Convert quaternion to Euler angles (in radians)
        let ( yaw, pitch,  roll) = transform.rotation.to_euler(EulerRot::YXZ);

        // Extract scale directly
        let scale = transform.scale;

        // Create and return a new instance of TransformSimple
        TransformSimple {
            translation,
            rotation: Vec3::new(yaw * (180.0 / PI), pitch* (180.0 / PI), roll * (180.0 / PI)),   //must be in degrees ! 
            scale,
        }
    }
}



impl TransformSimple {
    pub fn lerp(&self, other: &TransformSimple, factor: f32) -> TransformSimple {
        TransformSimple {
            translation: self.translation.lerp(other.translation, factor),
            rotation: self.rotation.lerp(other.rotation, factor),
            scale: self.scale.lerp(other.scale, factor),
        }
    }

    //assumes the input is degrees !!
    pub fn to_transform(&self) -> Transform {
        // Convert Euler angles (yaw, pitch, roll) back to a quaternion for rotation
        let yaw = self.rotation.x * (PI / 180.0);
        let pitch = self.rotation.y * (PI / 180.0);
        let roll = self.rotation.z * (PI / 180.0);

        let quat = Quat::from_euler(bevy::math::EulerRot::YXZ, yaw, pitch, roll);

        // Construct and return the Transform
        Transform {
            translation: self.translation,
            rotation: quat,
            scale: self.scale,
        }
    }


    pub fn mul_transform_simple(&self , rhs : &TransformSimple ) -> TransformSimple {
        // Convert Euler angles to quaternions for rotation
        let lhs_quat = Quat::from_euler(EulerRot::YXZ, self.rotation.x, self.rotation.y, self.rotation.z);
        let rhs_quat = Quat::from_euler(EulerRot::YXZ, rhs.rotation.x, rhs.rotation.y, rhs.rotation.z);

        // Combine scales
        let scale = self.scale * rhs.scale;

        // Rotate the lhs translation by the rhs rotation, then scale it
        let rotated_translation = rhs_quat * self.translation;
        let scaled_translation = rotated_translation * rhs.scale;

        // Combine translations
        let translation = scaled_translation + rhs.translation;

        // Combine rotations
        let rotation_quat = lhs_quat * rhs_quat;

        // Convert combined quaternion back to Euler angles
        let (yaw, pitch, roll) = rotation_quat.to_euler(EulerRot::YXZ);
        let rotation = Vec3::new(yaw, pitch, roll);

        TransformSimple {
            translation,
            rotation,
            scale,
        }



    }
}
