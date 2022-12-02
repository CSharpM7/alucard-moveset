use super::*;
const LENGTH: f32 = 15.0;
const ORIGIN_X: f32 = 8.0;
const ORIGIN_Y: f32 = 2.0;
const ORIGIN_Y_OFFSET: f32 = 3.0;
const ANGLE_OFFSET: f32 = 4.0;


const DAMAGE: f32 = 8.0;
const BKB: i32 = 70;
const KBG: i32 = 55;

#[acmd_script( agent = "richter", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE+2.0, 100, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y, LENGTH+ORIGIN_X, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("top"), DAMAGE, 80, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y, ORIGIN_X, Some(0.0), Some(ORIGIN_Y), Some(LENGTH+ORIGIN_X), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        
        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE/5.0, 70, KBG-4, 0, BKB, 3.5, 0.0, ORIGIN_Y, ORIGIN_X-2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);  
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}
#[acmd_script( agent = "richter", script = "effect_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_lw3_game,
        richter_attack_lw3_effect
    );
}
