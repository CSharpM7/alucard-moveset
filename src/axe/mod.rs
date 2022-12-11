use super::*;
mod fly;
mod hop;

pub fn install() {
    fly::install();
    hop::install();
    smashline::install_agent_frames!(
        axe_update
    );
}


#[weapon_frame( agent = WEAPON_KIND_RICHTER_AXE )]
fn axe_update(weapon: &mut L2CFighterBase) {
    unsafe {
        let has_hit = AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) ||
        AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_SHIELD);

        if (has_hit && !weapon.is_status(*WEAPON_SIMON_AXE_STATUS_KIND_HOP))
        {
            println!("KABOOM");
            weapon.change_status(WEAPON_SIMON_AXE_STATUS_KIND_HOP.into(), false.into());
        }
    }
}
