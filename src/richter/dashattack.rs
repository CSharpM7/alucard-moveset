use super::*;

#[acmd_script( agent = "richter", script = "game_attackdash" , category = ACMD_GAME )]
unsafe fn richter_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.6);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(fighter, 1);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 52, 88, 0, 70, 7.0, 0.0, 10.0, 14.0, Some(0.0), Some(10.0), Some(16.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 52, 80, 0, 70, 5.0, 0.0, 8.0, 11.5, Some(0.0), Some(8.0), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
    }
    wait(fighter.lua_state_agent, 2.0);
    FT_MOTION_RATE(fighter, 0.8);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 45, 110, 0, 60, 2.0, 0.0, 9.0, 21.5, Some(0.0), Some(15.0), Some(9.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, /*ID*/ 1,false);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "richter", script = "effect_attackdash" , category = ACMD_EFFECT )]
unsafe fn richter_attack_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 17.0);
    if is_excute(fighter) {
    LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_dash_game,
        richter_attack_dash_effect
    );
}

