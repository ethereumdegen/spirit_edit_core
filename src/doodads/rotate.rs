


use crate::doodads::doodad::RotateByDegrees;
use bevy::prelude::*;



pub fn handle_rotate_by_degrees(

	mut commands: Commands, 
	mut doodad_query:  Query<(Entity, &mut Transform, &RotateByDegrees)>

){

	for (doodad_entity, mut doodad_xform, rotate_comp) in doodad_query.iter_mut(){


		let deg = rotate_comp.0;

		let rads =  deg.to_radians() ;


		doodad_xform.rotate_local_y( rads );

		commands.get_entity(doodad_entity).map(|mut cmd|  
		{ cmd.remove::<RotateByDegrees>(); }

	  );


	}



}