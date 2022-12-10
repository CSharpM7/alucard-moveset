use super::*;
const ORIGIN_Z: f32 = 20.5;
const END_Y: f32 = 10.0;
const LENGTH: f32 = 18.25;
const FRAME: f32 = 25.0;

#[acmd_script( agent = "richter", script = "game_attackhi4" , category = ACMD_GAME )]
unsafe fn richter_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    let kbg=100;
    let fkb=135;
    let z=6.5;
    let angle=15;
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 3.0, 90, kbg, fkb, 0, 3.0, 0.0, 0.0, 0.0, None,None,None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("throw"), 3.0, 90-angle, kbg, fkb, 0, 3.0, 0.0, 0.0, -z, Some(0.0), Some(0.0),Some(-z/2.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("throw"), 3.0, 90+angle, kbg, fkb, 0, 3.0, 0.0, 0.0, z, Some(0.0), Some(0.0),Some(z/2.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(fighter,0, 2.0);
        ATK_SET_SHIELD_SETOFF_MUL(fighter,1, 2.0);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear(boma,0,false);
        AttackModule::clear(boma,2,false);
        AttackModule::clear(boma,3,false);
        ATTACK(fighter, 1, 0, Hash40::new("throw"), 3.0, 90, kbg, fkb/5, 0, 3.5, 0.0, 3.0, 0.0, Some(0.0), Some(LENGTH-4.0), Some(0.0), 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, FRAME);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 15.0, 86, 91, 0, 40, 3.5, 0.0, LENGTH/2.0, 0.0, Some(0.0), Some(LENGTH), Some(0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("throw"), 13.0, 86, 91, 0, 40, 3.5, 0.0, 3.0, 0.0, Some(0.0), Some(LENGTH/2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);

        search!(fighter,*MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackhi4" , category = ACMD_EFFECT )]
unsafe fn richter_attack_hi4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 18, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_teamhealfield"), Hash40::new("top"), 0,0,ORIGIN_Z,0,0,0, 0.9, true);
        LAST_EFFECT_SET_COLOR(fighter,1.0,0.25,1.0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_assist_out"), Hash40::new("top"), 0,0,ORIGIN_Z,0,0,0, 0.75, true);
        LAST_EFFECT_SET_RATE(fighter,0.9);
    }
    frame(lua_state, FRAME-1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("throw"), 0, 20, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_teamhealfield"), false,false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_assist_out"), Hash40::new("top"), 0,END_Y,ORIGIN_Z,0,0,0, 0.875, true);
        LAST_EFFECT_SET_RATE(fighter,1.75);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackhi4charge" , category = ACMD_EFFECT )]
unsafe fn richter_attack_hi4charge_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 5.0);
    for f in 1..20 
    {
        if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);

        //EFFECT_FOLLOW(fighter, Hash40::new("sys_teamhealfield"), Hash40::new("top"), 0,0,ORIGIN_Z,0,0,0, 0.75, true);
        //LAST_EFFECT_SET_COLOR(fighter,1.0,0.25,1.0);
        }
        else {break;}

        wait(lua_state, 5.0);
        if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("handl"), 2.0, 0, -2.5, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        wait(lua_state, 5.0);
        }
        else {break;}
    }
}


pub fn install() {
    install_acmd_scripts!(
        richter_attack_hi4_game,
        richter_attack_hi4_effect,
        richter_attack_hi4charge_effect
    );
}

