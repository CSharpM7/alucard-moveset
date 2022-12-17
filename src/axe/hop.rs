use super::*;
const MOTION_FRAMES: f32 = 10.0;

#[acmd_script( agent = "richter_axe", script = "game_hoph" , category = ACMD_GAME )]
unsafe fn axe_hoph_game(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    
    let maxFrame =  WorkModule::get_param_int(weapon.module_accessor, hash40("param_axe"), hash40("hop_life")) as f32;
    
    if is_excute(weapon) {
        ATTACK(weapon, 0, 0, Hash40::new("axe"), 1.0, 60, 15, 0, 35, 5.0, 0.0, -1.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_MAGIC);
        
        ATK_SET_SHIELD_SETOFF_MUL(weapon, 0, 1.25);
        AttackModule::set_add_reaction_frame_revised(weapon.module_accessor,  0, 3.0, false);
    }
    frame(lua_state, maxFrame-3.0);
    if is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
    }
    frame(lua_state, maxFrame-2.0);
    if is_excute(weapon) {
        ATTACK(weapon, 0, 0, Hash40::new("top"), 13.0, 62, 68, 0, 82, 15.0, 0.0, 6.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        
        ATK_SET_SHIELD_SETOFF_MUL(weapon, 0, 1.1);
    }
    wait(lua_state,2.0);
    if is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
    }
}
#[acmd_script( agent = "richter_axe", script = "effect_hoph" , category = ACMD_EFFECT )]
unsafe fn axe_hoph_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let maxFrame =  WorkModule::get_param_int(weapon.module_accessor, hash40("param_axe"), hash40("hop_life")) as f32;
    frame(lua_state, maxFrame-2.0);
    if is_excute(weapon) {
        EFFECT(weapon, Hash40::new("sys_bomb_a"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 0.8, 0,0,0,0,0,0,true);
    }
}

#[acmd_script( agent = "richter_axe", script = "sound_hoph" , category = ACMD_SOUND )]
unsafe fn axe_hoph_sound(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let maxFrame =  WorkModule::get_param_int(weapon.module_accessor, hash40("param_axe"), hash40("hop_life")) as f32;
    frame(lua_state, maxFrame-2.0);
    if is_excute(weapon) {
        PLAY_SE(weapon, Hash40::new("se_richter_special_s02_smash"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        axe_hoph_game,
        axe_hoph_effect,
        axe_hoph_sound
    );
}

