use super::*;
const LENGTH: f32 = 15.0;

#[acmd_script( agent = "richter_whip", script = "game_attackhold" , category = ACMD_GAME , low_priority)]
unsafe fn whip_attack_hold(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    /*
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, PH2*NDARY_CRAW_MOVE);
        battle_object();
        methodlib::L2CValue::L2CValue(void*)();
    }
    else{
        methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
        }
    methodlib::L2CValue::as_pointer()const();
    reset_node_fix_flag_list();
    battle_object();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(5, 11, 12);
    set_node_fix_flag_list();
    */
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("whip"), 1.0, 80, 35, 0, 40, 2.2, 0.0, 0.0, 0.0, Some(0.0), Some(LENGTH), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 13, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
}

pub fn install() {
    install_acmd_scripts!(
        whip_attack_hold
    );
}
