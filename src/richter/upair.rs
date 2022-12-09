use super::*;

#[acmd_script( agent = "richter", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
    WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
    JostleModule::set_status(boma, false);
    }
    let startY = 17.0;
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 90, 30, 0, 80, 3.4, 0.0, startY, -2.0, Some(0.0), Some(startY), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);

        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 130, 30, 0, 80, 3.4, 0.0, startY, 12.0, Some(0.0), Some(startY), Some(4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 50, 30, 0, 80, 3.4, 0.0, startY, -12.0, Some(0.0), Some(startY), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 150, 30, 0, 80, 3.4, 0.0, 22.2, 7.4, Some(0.0), Some(startY), Some(4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 30, 30, 0, 80, 3.4, 0.0, 22.2, -7.4, Some(0.0), Some(startY), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }


    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 86, 64, 0, 70, 4.5, 0.0, 38.5, 1.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);

        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 86, 58, 0, 70, 7.0, 0.0, 20.5, 1.8, Some(0.0), Some(28.5), Some(1.8), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
    AttackModule::clear(boma, /*ID*/ 0,false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
    WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackairhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 3, 15, 5, -90, 0, 0, 1.2, true, *EF_FLIP_YZ);
        //LAST_PARTICLE_SET_COLOR(fighter, 0.254, 0.024, 0.104);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4, 15, -3, -90, 0, 0, 1.2, true, *EF_FLIP_YZ);
        //LAST_PARTICLE_SET_COLOR(fighter, 0.254, 0.024, 0.104);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_air_hi_game,
        richter_attack_air_hi_effect
    );
}
