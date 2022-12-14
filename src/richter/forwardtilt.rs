use super::*;
const ORIGIN_X: f32 = 2.0;
const ORIGIN_Y: f32 = 2.0;
const ORIGIN_Y_OFFSET: f32 = 3.0;
const ANGLE_OFFSET: f32 = 4.0;


const DAMAGE: f32 = 10.0;
const BKB: i32 = 65;
const KBG: i32 = 65;

/*
#[acmd_script( agent = "richter", script = "game_attacks3" , category = ACMD_GAME )]
unsafe fn richter_attack_s3_game_staight(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 3.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), DAMAGE+2.0, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y, vars::LENGTH+ORIGIN_X, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("top"), DAMAGE, 361, KBG, 0, BKB, 2.3, 0.0, ORIGIN_Y, ORIGIN_X, Some(0.0), Some(ORIGIN_Y), Some(vars::LENGTH+ORIGIN_X), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        
        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE/5.0, 361, KBG-4, 0, BKB, 3.5, 0.0, ORIGIN_Y, ORIGIN_X-2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);    

        search!(fighter,*MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}
*/
#[acmd_script( agent = "richter", script = "game_attacks3" , category = ACMD_GAME )]
unsafe fn richter_attack_s3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 3.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), DAMAGE+2.0, 361, KBG, 0, BKB, vars::WIDTH+0.2, 0.0, vars::LENGTH+ORIGIN_X,0.0, Some(0.0), Some(vars::LENGTH+ORIGIN_X-3.0),Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("haver"), DAMAGE, 361, KBG, 0, BKB, vars::WIDTH, 0.0, ORIGIN_X, 0.0, Some(0.0), Some(vars::LENGTH+ORIGIN_X-3.0),Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        
        ATTACK(fighter, 2, 0, Hash40::new("top"), DAMAGE/5.0, 361, KBG-4, 0, BKB, 4.5, 0.0,  7.0, 9.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);    

        search!(fighter,*MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_s3_game
    );
}
