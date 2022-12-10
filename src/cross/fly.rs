use super::*;

#[acmd_script( agent = "richter_cross", script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn cross_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    /*
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 5.0, 75, 95, 0, 55, 1.2, 0.0, 3.7, 0.0, 0.0, -3.7, 0.0, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 5.0, 75, 95, 0, 55, 1.2, 0.0, 0.0, 3.7, 0.0, 0.0, -3.7, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
    }
    */
}

#[acmd_script( agent = "richter_cross", script = "game_turn" , category = ACMD_GAME , low_priority)]
unsafe fn cross_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 10.0, 80, 95, 0, 55, 1.2, 0.0, 0.0, 0.0, Some(0.0), Some(20.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
}
#[acmd_script( agent = "richter_cross", script = "effect_fly" , category = ACMD_EFFECT , low_priority)]
unsafe fn cross_fly_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script( agent = "richter_cross", script = "effect_turn" , category = ACMD_EFFECT , low_priority)]
unsafe fn cross_turn_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}

pub fn install() {
    install_acmd_scripts!(
        cross_fly_game,
        cross_fly_effect,

        cross_turn_game,
        cross_turn_effect,
    );
}

