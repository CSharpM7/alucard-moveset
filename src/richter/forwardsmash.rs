use super::*;
const LENGTH: f32 = 15.0;
const ORIGIN_X: f32 = 23.0;
const ORIGIN_Y: f32 = 7.5;
const ANGLE_OFFSET: f32 = 7.5;

const DAMAGE: f32 = 15.0;
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
        
        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE+2.0, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y+ANGLE_OFFSET, LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0), None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("top"), DAMAGE, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y+(ANGLE_OFFSET/3.0), ORIGIN_X, Some(0.0), Some(ORIGIN_Y+ANGLE_OFFSET), Some(LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0)), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE/5.0, 361, KBG-4, 0, BKB, 3.5, 0.0, ORIGIN_Y+(ANGLE_OFFSET/3.0), ORIGIN_X-2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);     
    }
    if (AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)){
        DamageModule::heal(fighter.module_accessor, 10.0,0);
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
        
        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE+2.0, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y, LENGTH+ORIGIN_X, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("top"), DAMAGE, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y, ORIGIN_X, Some(0.0), Some(ORIGIN_Y), Some(LENGTH+ORIGIN_X), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE/5.0, 361, KBG-4, 0, BKB, 3.5, 0.0, ORIGIN_Y, ORIGIN_X-2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);     
    }
    if (AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)){
        DamageModule::heal(fighter.module_accessor, 10.0,0);
    }
    wait(lua_state, 3.0);
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
        
        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE+2.0, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y-ANGLE_OFFSET, LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0), None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("top"), DAMAGE, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y-(ANGLE_OFFSET/3.0), ORIGIN_X, Some(0.0), Some(ORIGIN_Y-ANGLE_OFFSET), Some(LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0)), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE/5.0, 361, KBG-4, 0, BKB, 3.5, 0.0, ORIGIN_Y-(ANGLE_OFFSET/3.0), ORIGIN_X-2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);     
    }
    if (AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)){
        DamageModule::heal(fighter.module_accessor, 10.0,0);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter", script = "sound_attacks4hi" , category = ACMD_SOUND )]
unsafe fn richter_attack_s4_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s01"));
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack05"));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s02"));
    }
}

#[acmd_script( agent = "richter", script = "sound_attacks4" , category = ACMD_SOUND )]
unsafe fn richter_attack_s4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s01"));
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack05"));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s02"));
    }
}

#[acmd_script( agent = "richter", script = "sound_attacks4lw" , category = ACMD_SOUND )]
unsafe fn richter_attack_s4_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s01"));
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack05"));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_s02"));
    }
}

#[acmd_script( agent = "richter", script = "effect_attacks4hi" , category = ACMD_EFFECT )]
unsafe fn richter_attack_s4_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("haver"), 0, 0, 0, 0, 65, 0, 1.3, true);
    }
}
#[acmd_script( agent = "richter", script = "effect_attacks4" , category = ACMD_EFFECT )]
unsafe fn richter_attack_s4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("haver"), 0, 0, 0, 0, 65, 0, 1.3, true);
    }
}
#[acmd_script( agent = "richter", script = "effect_attacks4lw" , category = ACMD_EFFECT )]
unsafe fn richter_attack_s4_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("haver"), 0, 0, 0, 0, 65, 0, 1.3, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_s4_hi_game,
        richter_attack_s4_hi_sound,
        richter_attack_s4_hi_effect,
        richter_attack_s4_game,
        richter_attack_s4_sound,
        richter_attack_s4_effect,
        richter_attack_s4_lw_game,
        richter_attack_s4_lw_sound,
        richter_attack_s4_lw_effect
    );
}
