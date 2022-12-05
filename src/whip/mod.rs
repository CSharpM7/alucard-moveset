use super::*;
mod backair;
mod downtilt;

pub fn install() {
    backair::install();
    downtilt::install();
    smashline::install_agent_frames!(
        whip_update
    );
}



#[weapon_frame( agent = WEAPON_KIND_RICHTER_WHIP )]
fn whip_update(weapon: &mut L2CFighterBase) {
    unsafe {
        let mut backAir = weapon.is_motion(Hash40::new("attack_air_b_hi")) ||
        weapon.is_motion(Hash40::new("attack_air_b")) ||
        weapon.is_motion(Hash40::new("attack_air_b_lw"));
        
        backAir = backAir && weapon.motion_frame() <= 35.0;

        ModelModule::set_mesh_visibility(weapon.module_accessor, 
            Hash40::new("stake"), 
            backAir
        );
        
    }
}