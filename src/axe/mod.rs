use super::*;
mod fly;
mod hop;

pub fn install() {
    fly::install();
    hop::install();
    //smashline::install_agent_frames!(
    //    axe_update
    //);
    install_status_scripts!(
        axe_fly_exec
    );
}


#[status_script(agent = "richter_axe", status = WEAPON_SIMON_AXE_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn axe_fly_exec(weapon: &mut L2CFighterCommon) -> L2CValue {

    let has_hit_shield = AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_SHIELD);
    let has_hit = AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ATTACK)
    || has_hit_shield;

    let maxFrame =  WorkModule::get_param_int(weapon.module_accessor, hash40("param_axe"), hash40("life")) as f32;
    if (has_hit ||
        weapon.motion_frame() >= maxFrame-2.0)
    {
        weapon.change_status(WEAPON_SIMON_AXE_STATUS_KIND_HOP.into(), false.into());
    }
    return 0.into()
}

#[weapon_frame( agent = WEAPON_KIND_RICHTER_AXE )]
fn axe_update(weapon: &mut L2CFighterBase) {
    unsafe {
        let has_hit_shield = AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_SHIELD);
        let has_hit = AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ATTACK)
        || has_hit_shield;

        let maxFrame =  WorkModule::get_param_int(weapon.module_accessor, hash40("param_axe"), hash40("life")) as f32;
        if weapon.is_status(*WEAPON_SIMON_AXE_STATUS_KIND_FLY)
        {
            if (has_hit ||
             weapon.motion_frame() >= maxFrame-2.0)
            {
                weapon.change_status(WEAPON_SIMON_AXE_STATUS_KIND_HOP.into(), false.into());
            }
        }
    }
}
