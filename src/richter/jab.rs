use super::*;

#[acmd_script( agent = "richter", script = "game_attack11" , category = ACMD_GAME )]
unsafe fn richter_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;frame(lua_state, 5.0);
    frame(lua_state, 0.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 2.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 10, 0, 10, 3.2, 0.0, 11.0, 6.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 180, 10, 0, 10, 3.2, 0.0, 11.0, 11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 5.0, /*Unk*/ false);

        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

#[acmd_script( agent = "richter", script = "sound_attack11" , category = ACMD_SOUND )]
unsafe fn richter_attack_11_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        //PLAY_SE(fighter, Hash40::new("se_richter_swing_s"));
        PLAY_SE(fighter, Hash40::new("se_richter_attackair_n01"));
    }
}
#[acmd_script( agent = "richter", script = "effect_attack11" , category = ACMD_EFFECT )]
unsafe fn richter_attack_11_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 14.5, 4, 0, -10, 0, 0.35, true, *EF_FLIP_YZ, 0.6);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), -1, 14.5, 12, 0, 0, 0, 0.5, true, *EF_FLIP_YZ, 0.5);
    }
}

#[acmd_script( agent = "richter", script = "game_attack12" , category = ACMD_GAME )]
unsafe fn richter_attack_12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 3.0, 0.0, 9.5, 3.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 3.0, 0.0, 9.5, 7.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 3.5, 0.0, 9.5, 11.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 7.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 7.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 7.0, /*Unk*/ false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
}
#[acmd_script( agent = "richter", script = "sound_attack12" , category = ACMD_SOUND )]
unsafe fn richter_attack_12_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_swing_m"));
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_step_right_s"));
    }
}
#[acmd_script( agent = "richter", script = "effect_attack12" , category = ACMD_EFFECT )]
unsafe fn richter_attack_12_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("top"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if is_excute(fighter) {
        if(sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR)==0.0){;
            EFFECT_FOLLOW_arg11(fighter,Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 9.5, 2.5, -15, -70, 35, 0.9, true, 1);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
        else{
            EFFECT_FOLLOW_arg11(fighter,Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 9, 3, -14, -67, 33, 0.9, true, 1);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    }

}


#[acmd_script( agent = "richter", script = "game_attack13" , category = ACMD_GAME )]
unsafe fn richter_attack_13_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;frame(lua_state, 5.0);

    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 30, 50, 0, 75, 4.5, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}
#[acmd_script( agent = "richter", script = "sound_attack13" , category = ACMD_SOUND )]
unsafe fn richter_attack_13_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_whip_holding"));
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_m"));
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_step_right_s"));
    }
}

#[acmd_script( agent = "richter", script = "effect_attack13" , category = ACMD_EFFECT )]
unsafe fn richter_attack_13_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), true, true);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW_WORK(fighter, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.4);
        //AFTER_IMAGE4_ON_arg29(Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 3, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("top"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        //AFTER_IMAGE_OFF(fighter, 4);
        //EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

#[acmd_script( agent = "richter", script = "expression_attack13" , category = ACMD_EXPRESSION )]
unsafe fn richter_attack_13_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if is_excute(fighter) {
        //methodlib::L2CValue::as_hash()const(Hash40::new("haver"), *ATTACK_DIRECTION_Z, *ATTACK_DIRECTION_Y, *ATTACK_DIRECTION_X);
        //AttackModule::set_attack_reference_joint_id(boma);
        slope!(fighter,*MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor,Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashs"), 0);
        ControlModule::set_rumble(fighter.module_accessor,Hash40::new("rbkind_nohits"), 0, false, 0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        slope!(fighter,*MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}



#[acmd_script( agent = "richter", script = "game_attack100" , category = ACMD_GAME )]
unsafe fn richter_attack_100_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    frame(lua_state, 1.0);
    for i in 1..100000{
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.75, 361, 10, 0, 5, 7.3, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(16.0), 0.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        wait(lua_state, 2.0);
        for j in 1..5{
            if is_excute(fighter) {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 0.75, 361, 10, 0, 5, 7.3, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(16.0), 0.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 7.0, /*Unk*/ false);
            }
            wait(lua_state, 2.0);
            if is_excute(fighter) {
                AttackModule::clear_all(boma);
                WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
            }
            wait(lua_state, 2.0);
        }
    }
}

#[acmd_script( agent = "richter", script = "sound_attack100" , category = ACMD_SOUND )]
unsafe fn richter_attack_100_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    //macros::PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
    frame(lua_state, 1.0);
    for i in 1..100000{
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_richter_attack100_01"));
        }
        wait(lua_state, 5.0);
    }
}

#[acmd_script( agent = "richter", script = "effect_attack100" , category = ACMD_EFFECT )]
unsafe fn richter_attack_100_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    for i in 1..100000{
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("richter_attack100_end"), Hash40::new("top"), 0, 8, 6, 5, 0, -40, 0.825, true);
            LAST_EFFECT_SET_RATE(fighter,2.5);
        } else {break;}
        wait(lua_state, 1.5);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("richter_attack100_end"), Hash40::new("top"), 0, 8, 12, 5, 0, -35, 0.7, true);
            LAST_EFFECT_SET_RATE(fighter,2.5);
        } else {break;}
        wait(lua_state, 1.5);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("richter_attack100_end"), Hash40::new("top"), 0, 8, 6, 0, 0, 30, 0.825, true);
            LAST_EFFECT_SET_RATE(fighter,2.5);
        }
        else {break;}
        wait(lua_state, 1.5);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("richter_attack100_end"), Hash40::new("top"), 0, 8, 12, 0, 0, 25, 0.7, true);
            LAST_EFFECT_SET_RATE(fighter,2.5);
        } else {break;}
        wait(lua_state, 1.5);
    }
}



#[acmd_script( agent = "richter", script = "game_attack100end" , category = ACMD_GAME )]
unsafe fn richter_attack_100end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;frame(lua_state, 5.0);

    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 50, 120, 0, 60, 7.5, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(18.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}
#[acmd_script( agent = "richter", script = "sound_attack100end" , category = ACMD_SOUND )]
unsafe fn richter_attack_100end_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    //macros::PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_attack100_02"));
    }
}

#[acmd_script( agent = "richter", script = "effect_attack100end" , category = ACMD_EFFECT )]
unsafe fn richter_attack_100end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    wait(lua_state, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("richter_attack100_end"), Hash40::new("top"), 0, 11, 11, 0, 0, 0, 0.75, true);
    }
}


pub fn install() {
    install_acmd_scripts!(
        richter_attack_11_game,
        richter_attack_11_sound,
        richter_attack_11_effect,

        richter_attack_12_game,
        richter_attack_12_sound,
        richter_attack_12_effect,

        richter_attack_13_game,
        richter_attack_13_sound,
        richter_attack_13_effect,
        richter_attack_13_expression,

        richter_attack_100_game,
        //richter_attack_100_sound,
        richter_attack_100_effect,

        richter_attack_100end_game,
        richter_attack_100end_effect
    );
}
