
use serde::Serialize;

use serde::Deserialize;

#[derive(Clone,Debug,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub enum MaterialOverrideLayer {

	Base

}




impl MaterialOverrideLayer {

	pub fn get_material_layer_name(&self) -> String {


		match &self {
			Self::Base => "base_material", 
		}.into()

	}

}



#[derive(Clone,Debug,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub enum MaterialOverrideType {
	Stone1,
	Stone2,
	Stone3, 
}

impl MaterialOverrideType {

	pub fn get_material_name(&self) -> String {


		match &self {
			Self::Stone1 => "Stone1",
			Self::Stone2 => "Stone2",
			Self::Stone3 => "Stone3"
		}.into()

	}

}
