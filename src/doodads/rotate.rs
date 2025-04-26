

#[derive(Component, Debug,Default)]
pub struct RotateByDegrees( pub Vec3 );

//use crate::doodads::doodad::RotateByDegrees;
use bevy::prelude::*;



// yaw is Y 

// pitch is X 



impl EntityCommand for RotateByDegrees {



	fn apply(self, mut entity_world: EntityWorldMut   ) {  

 

		if let Some(mut xform) = entity_world.get_mut::<Transform>( ) {


			let degrees_amounts = self.0; 
			//let deg = rotate_comp.0;

		//let rads =  deg.to_radians() ;

			xform.rotate_local_x( degrees_amounts.x.to_radians() );
			xform.rotate_local_y( degrees_amounts.y.to_radians() );

			xform.rotate_local_z( degrees_amounts.z.to_radians() );

		}



	}
}

/*

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



}*/