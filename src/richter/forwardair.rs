use super::*;
const LENGTH: f32 = 15.0;
const ORIGIN_X: f32 = 9.0;
const ORIGIN_Y: f32 = 9.0;
const ORIGIN_Y_OFFSET: f32 = 3.0;
const ANGLE_OFFSET: f32 = 4.0;


const DAMAGE: f32 = 10.0;
const BKB: i32 = 55;
const KBG: i32 = 80;

#[acmd_script( agent = "richter", script = "game_attackairfhi" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_f_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 2.3, 0.0, ORIGIN_Y+ANGLE_OFFSET, ORIGIN_X-2.0, Some(0.0), Some(ORIGIN_Y+ANGLE_OFFSET+ORIGIN_Y_OFFSET), Some(LENGTH+ORIGIN_X), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE+2.0, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y+ANGLE_OFFSET+ORIGIN_Y_OFFSET, LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0), None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("top"), DAMAGE, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y+ORIGIN_Y_OFFSET, ORIGIN_X, Some(0.0), Some(ORIGIN_Y+ANGLE_OFFSET+ORIGIN_Y_OFFSET), Some(LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0)), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE/5.0, 361, KBG-4, 0, BKB, 3.5, 0.0, ORIGIN_Y+ORIGIN_Y_OFFSET, ORIGIN_X-2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);     

        search!(fighter,*MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "richter", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 2.3, 0.0, ORIGIN_Y, ORIGIN_X-2.0, Some(0.0), Some(ORIGIN_Y), Some(LENGTH+ORIGIN_X), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE+2.0, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y, LENGTH+ORIGIN_X, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("top"), DAMAGE, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y, ORIGIN_X, Some(0.0), Some(ORIGIN_Y), Some(LENGTH+ORIGIN_X), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        
        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE/5.0, 361, KBG-4, 0, BKB, 3.5, 0.0, ORIGIN_Y, ORIGIN_X-2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);    

        search!(fighter,*MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "richter", script = "game_attackairflw" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_f_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);

    frame(lua_state, 14.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 2.3, 0.0, ORIGIN_Y-ANGLE_OFFSET-2.0, ORIGIN_X-2.0, Some(0.0), Some(ORIGIN_Y-ANGLE_OFFSET-ORIGIN_Y_OFFSET-2.0), Some(LENGTH+ORIGIN_X), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE+2.0, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y-ANGLE_OFFSET-ORIGIN_Y_OFFSET-2.0, LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0), None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        
        ATTACK(fighter, 1, 0, Hash40::new("top"), DAMAGE, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y-ORIGIN_Y_OFFSET-2.0, ORIGIN_X, Some(0.0), Some(ORIGIN_Y-ANGLE_OFFSET-ORIGIN_Y_OFFSET-2.0), Some(LENGTH+ORIGIN_X-(ANGLE_OFFSET/5.0)), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        
        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE/5.0, 361, KBG-4, 0, BKB, 3.5, 0.0, ORIGIN_Y-ORIGIN_Y_OFFSET-2.0, ORIGIN_X-2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);    
        
        search!(fighter,*MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_air_f_hi_game,
        richter_attack_air_f_game,
        richter_attack_air_f_lw_game
    );
}
