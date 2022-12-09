use super::*;

#[acmd_script( agent = "richter", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.2);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(fighter, 1);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 21.0);
    FT_MOTION_RATE(fighter, 1);
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 3.0, 0.0, 18.0, 2.0, Some(0.0), Some(52.0), Some(2.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 90, 91, 0, 64, 3.0, 0.0, 52.0, 2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 90, 85, 0, 63, 3.0, 0.0, 18.0, 2.0, Some(0.0), Some(52.0), Some(2.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);

        search!(fighter,*MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_hi4_game
    );
}

