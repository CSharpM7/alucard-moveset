use super::*;

#[acmd_script( agent = "richter_axe", script = "game_hopv" , category = ACMD_GAME )]
unsafe fn axe_hopv_game(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let maxFrame =  WorkModule::get_param_int(weapon.module_accessor, hash40("param_axe"), hash40("hop_life")) as f32;
    
    frame(lua_state, maxFrame-1.0);
    if is_excute(weapon) {
        ATTACK(weapon, 0, 0, Hash40::new("top"), 13.0, 62, 64, 0, 77, 14.0, 0.0, 0.0, 0.0, Some(0.0),Some(0.0),Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        
        ATK_SET_SHIELD_SETOFF_MUL(weapon, 0, 1.1);
    }
    wait(lua_state,1.0);
    if is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
    }
}
#[acmd_script( agent = "richter_axe", script = "effect_hopv" , category = ACMD_EFFECT )]
unsafe fn axe_hopv_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let maxFrame =  WorkModule::get_param_int(weapon.module_accessor, hash40("param_axe"), hash40("hop_life")) as f32;
    frame(lua_state, maxFrame-1.0);
    if is_excute(weapon) {
        EFFECT(weapon, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0,0,0,0,0,0,true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        axe_hopv_game,
        axe_hopv_effect
    );
}

