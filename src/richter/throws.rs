use super::*;

#[acmd_script( agent = "richter", script = "game_catch" , category = ACMD_GAME )]
unsafe fn richter_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 6.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.625);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1.0);

    frame(lua_state, 9.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 6.6, 2.35, Some(0.0), Some(6.6), Some(10.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        macros::game_CaptureCutCommon(fighter);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter,*MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ false);
    }
}

#[acmd_script( agent = "richter", script = "game_catchdash" , category = ACMD_GAME )]
unsafe fn richter_catch_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 4.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);

    frame(lua_state, 12.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(12.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 1.3, 0.0, 6.6, 2.7, Some(0.0), Some(6.6), Some(13.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        macros::game_CaptureCutCommon(fighter);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter,*MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ false);
    }
}

#[acmd_script( agent = "richter", script = "game_catchturn" , category = ACMD_GAME )]
unsafe fn richter_catch_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.5, 6.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1.0);

    frame(lua_state, 12.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 6.6, -2.35, Some(0.0), Some(6.6), Some(-18.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        macros::game_CaptureCutCommon(fighter);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter,*MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ false);
    }
}



#[acmd_script( agent = "richter", script = "game_throwlw" , category = ACMD_GAME )]
unsafe fn richter_throw_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 74, 112, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.25);
    frame(lua_state, 19.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 21.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        //FT_CATCH_STOP(fighter, 10, 1);
        CHECK_FINISH_CAMERA(fighter, 5, 0);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}
/*
#[acmd_script( agent = "richter", script = "game_throwf" , category = ACMD_GAME )]
unsafe fn richter_throw_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 28, 68, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 16, 3);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(boma);
    }
}
*/
#[acmd_script( agent = "richter", script = "game_throwf" , category = ACMD_GAME )]
unsafe fn richter_throw_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 20, 55, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("armr"), 8.0, 50, 70, 0, 100, 4.0, 7.0, 0.0, 0.0, Some(8.0), Some(-6.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        CHECK_FINISH_CAMERA(fighter, 24, 7);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter", script = "effect_throwf" , category = ACMD_EFFECT )]
unsafe fn richter_throw_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 10.0, 3.5, 0, 5, 0, 0.625, true);
    }
}
#[acmd_script( agent = "richter", script = "expression_throwf" , category = ACMD_EXPRESSION )]
unsafe fn richter_throw_f_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        slope!(fighter,*MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(fighter.module_accessor,Hash40::new("rbkind_attackm"), 0, false, 0);
    }
}

#[acmd_script( agent = "richter", script = "game_throwhi" , category = ACMD_GAME )]
unsafe fn richter_throw_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 90, 82, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);        
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 89, 100, 0, 55, 2.0, 0.0, 3.0, -7.0, Some(0.0), Some(50.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);

        CHECK_FINISH_CAMERA(fighter, 24, 7);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(boma);
    }
}


pub fn install() {
    install_acmd_scripts!(
        richter_catch_game,
        richter_catch_turn_game,
        richter_catch_dash_game,

        richter_throw_lw_game,
        richter_throw_f_game,
        richter_throw_hi_game
        //richter_throw_f_effect,
        //richter_throw_f_expression
    );
}
