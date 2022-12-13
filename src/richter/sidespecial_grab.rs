
use super::*;

#[acmd_script( agent = "richter", script = "game_specials1" , category = ACMD_GAME )]
unsafe fn richter_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 6.8, 6.7, 7.5, 3.8);
        }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        /*
        CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 7.5,None,None,None, *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 1.0, 0.0, 8.0, 7.2, None,None,None, *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_GA);
        */
        CATCH(fighter, 0, Hash40::new("top"), 8.0, 0.0, 8.0, 7.5,None,None,None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 1.0, 0.0, 8.0, 7.2, None,None,None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        grab!(fighter,*MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 7.0, 7.0, 7.5, 7.5);
    }
}
#[acmd_script( agent = "richter", script = "game_specialairs1" , category = ACMD_GAME )]
unsafe fn richter_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 6.8, 6.7, 7.5, 3.8);
        }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        /*
        CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 7.5,None,None,None, *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 1.0, 0.0, 8.0, 7.2, None,None,None, *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_GA);
        */
        CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 7.5,None,None,None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 1.0, 0.0, 8.0, 7.2, None,None,None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        grab!(fighter,*MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 7.0, 7.0, 7.5, 7.5);
    }
}

pub fn install() {
    install_acmd_scripts!(
        //richter_special_s_game,
        //richter_special_air_s_game
    );
}
