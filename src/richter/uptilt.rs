use super::*;

#[acmd_script( agent = "richter", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 7.0, 85, 75, 0, 60, 4.0, 1.7, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 7.0, 85, 75, 0, 60, 4.0, 2.3, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("waist"), 7.0, 85, 75, 0, 60, 4.0, 0.0, 0.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

        ATTACK(fighter, 3, 0, Hash40::new("handl"), 5.0, 100, 60, 0, 55, 5.0, 1.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
	wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(boma,2,false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma,HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear(boma,0,false);
        AttackModule::clear(boma,1,false);
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear(boma,3,false);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackhi3" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_hi3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 14.5, 4, 0, 90, 0, 0.35, true, *EF_FLIP_YZ, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("arml"), 2.3, 0.0, -1.0, 0, 0, 0, 1.75, true);
        LAST_EFFECT_SET_ALPHA(fighter,0.4);
        LAST_EFFECT_SET_COLOR(fighter,0.25,0.25,1);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 7, 19, 2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, false, 0.4);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_hi3_game,
        richter_attack_hi3_effect
    );
}
