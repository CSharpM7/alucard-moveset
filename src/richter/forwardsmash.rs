use std::convert::TryInto;

use super::*;
const ORIGIN_X: f32 = 11.5;
const ORIGIN_Y: f32 = 7.5;
const ANGLE_OFFSET: f32 = 8.5;

const DAMAGE: f32 = 10.0;
const DAMAGE_EX: f32 = 15.0;
const BKB: i32 = 60;
const KBG: i32 = 80;

#[acmd_script( agent = "richter", script = "game_attacks4hi" , category = ACMD_GAME )]
unsafe fn richter_attack_s4_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.5);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        
        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE_EX+2.0, 361, KBG, 0, BKB, 5.5, 0.0, ORIGIN_Y+ANGLE_OFFSET, vars::LENGTH+ORIGIN_X-(ANGLE_OFFSET/3.0), None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("top"), DAMAGE_EX, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y+(ANGLE_OFFSET/3.0), ORIGIN_X, Some(0.0), Some(ORIGIN_Y+ANGLE_OFFSET), Some(vars::LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0)), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE_EX/5.0, 361, KBG-4, 0, BKB, 4.5, 0.0, ORIGIN_Y+(ANGLE_OFFSET/3.0), ORIGIN_X-5.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);      
        
        //("collision_attr_saving_ryu")
        //AttackModule::set_damage_effect_mul_scale(boma, 0.1);
        //AttackModule::set_attack_level(fighter.module_accessor, 0, (*FIGHTER_RYU_SAVING_LV_3).try_into().unwrap());
        //AttackModule::set_attack_level(fighter.module_accessor, 1, (*FIGHTER_RYU_SAVING_LV_3).try_into().unwrap());
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter", script = "game_attacks4" , category = ACMD_GAME )]
unsafe fn richter_attack_s4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.5);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        
        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE, 361, KBG, 0, 40, 4.0, 0.0, ORIGIN_Y, vars::LENGTH+ORIGIN_X, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        AttackModule::set_paralyze_frame(boma, 0, -30, true);

        ATTACK(fighter, 1, 0, Hash40::new("top"), DAMAGE-2.0, 361, KBG-10, 0, BKB, 2.3, 0.0, ORIGIN_Y, ORIGIN_X, Some(0.0), Some(ORIGIN_Y), Some(vars::LENGTH+ORIGIN_X), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE_EX/5.0, 361, KBG-10, 0, BKB, 4.5, 0.0, ORIGIN_Y, ORIGIN_X-5.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);      
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear(boma,0,false);

        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE/1.5, 361, KBG, 0, BKB-30, 2.3, 0.0, ORIGIN_Y, ORIGIN_X, Some(0.0), Some(ORIGIN_Y), Some(vars::LENGTH+ORIGIN_X), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter", script = "game_attacks4lw" , category = ACMD_GAME )]
unsafe fn richter_attack_s4_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.5);
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        
        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE+1.0, 80, KBG-50, 0, BKB, 3.0, 0.0, ORIGIN_Y-ANGLE_OFFSET, vars::LENGTH+ORIGIN_X-(ANGLE_OFFSET/6.0), None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("top"), DAMAGE, 80, KBG-50, 0, BKB, 2.3, 0.0, ORIGIN_Y-(ANGLE_OFFSET/3.0), ORIGIN_X, Some(0.0), Some(ORIGIN_Y-ANGLE_OFFSET), Some(vars::LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0)), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE_EX/5.0, 361, KBG-4, 0, BKB, 4.5, 0.0, ORIGIN_Y-(ANGLE_OFFSET/3.0), ORIGIN_X-5.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);     

        AttackModule::set_ice_frame_mul(fighter.module_accessor, 0, 0.625, false);
        AttackModule::set_ice_frame_mul(fighter.module_accessor, 1, 0.625, false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


#[acmd_script( agent = "richter", scripts = ["sound_attacks4hi","sound_attacks4","sound_attacks4lw"] , category = ACMD_SOUND )]
unsafe fn richter_attack_s4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let FIRE = "attack_s4_hi";
    let ELECTRIC = "attack_s4_s";
    let ICE = "attack_s4_lw";
    
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s01"));
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if (fighter.is_motion(Hash40::new(ELECTRIC))) {
            PLAY_SE(fighter, Hash40::new("se_richter_smash_s02"));
        }
        else if (fighter.is_motion(Hash40::new(FIRE))) {
            PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
        }
        else if (fighter.is_motion(Hash40::new(ICE))) {
            PLAY_SE(fighter, Hash40::new("se_item_ice_crash"));
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack05"));
    }
}


#[acmd_script( agent = "richter", scripts = ["effect_attacks4hi","effect_attacks4","effect_attacks4lw"] , category = ACMD_EFFECT )]
unsafe fn richter_attack_s4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let FIRE = "attack_s4_hi";
    let ELECTRIC = "attack_s4_s";
    let ICE = "attack_s4_lw";
    let mut color = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
        
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        
        if (fighter.is_motion(Hash40::new(ELECTRIC))) {
            color.x = 1.0;
            color.y = 1.0;
        }
        else if (fighter.is_motion(Hash40::new(FIRE))) {
            color.x = 1.0;
            color.y = 0.5;
        }
        else if (fighter.is_motion(Hash40::new(ICE))) {
            color.y = 0.5;
            color.z = 1.0;
        }

        FLASH(fighter, color.x, color.y,color.z, 0.7);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);

        if (fighter.is_motion(Hash40::new(ELECTRIC))) {
            EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, ORIGIN_Y, vars::LENGTH+ORIGIN_X+5.0, 0, 0, 0, 0.75, 0,0,0,0,0,0,true);
            LAST_EFFECT_SET_RATE(fighter, 1.375);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (fighter.is_motion(Hash40::new(FIRE))) {     
            EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, ORIGIN_Y+ANGLE_OFFSET, vars::LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0), 0, 0, 0, 0.6, 0,0,0,0,0,0,true);
        }
        else if (fighter.is_motion(Hash40::new(ELECTRIC))) {
            
            EFFECT_FOLLOW(fighter, Hash40::new("sys_pasaran_spark"), Hash40::new("haver"), 0, (vars::LENGTH/2.0)+2.0, 0, 0, 0, 0, 0.3,true);
            //EFFECT(fighter, Hash40::new("sys_pasaran_spark"), Hash40::new("top"), 0, ORIGIN_Y, (vars::LENGTH/2.0)+ORIGIN_X, 0, 0, 0, 0.5, 0,0,0,0,0,0,true);
            LAST_EFFECT_SET_COLOR(fighter,1,1,0);
        }
        else if (fighter.is_motion(Hash40::new(ICE))) {
            EFFECT(fighter, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, ORIGIN_Y-ANGLE_OFFSET, vars::LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0), 0, 0, 0, 0.5, 0,0,0,0,0,0,true);
        }
        FLASH(fighter, color.x, color.y,color.z, 0.7);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_pasaran_spark"),false,false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_flame"),false,false);
    }
}
pub fn install() {
    install_acmd_scripts!(
        richter_attack_s4_hi_game,
        richter_attack_s4_game,
        richter_attack_s4_lw_game,

        richter_attack_s4_sound,
        richter_attack_s4_effect
    );
}
