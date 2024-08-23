
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
	Wall1,
	Rocks1,
	Rocks2,
	 

	StoneToon1,
	StoneToon2,
	DirtToon1,
	Leaves1,
}

impl MaterialOverrideType {

	pub fn get_material_name(&self) -> String {


		match &self {
			Self::Wall1 => "Wall1",
			Self::Rocks1 => "Rocks1",
			Self::Rocks2 => "Rocks2",
		 
			Self::StoneToon1 => "StoneToon1",
			Self::StoneToon2 => "StoneToon2",
			Self::DirtToon1 => "DirtToon1",
			Self::Leaves1 => "Leaves1"
		}.into()

	}

}
