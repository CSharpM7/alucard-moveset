use super::*;
#[acmd_script( agent = "richter", scripts = ["game_finalstart","game_finalairstart"] , category = ACMD_GAME )]
unsafe fn richter_final_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;


    if is_excute(fighter) {
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 4.0, 50.0);
    }
    
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA){
        return;
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        SlowModule::set_whole(fighter.module_accessor, 2, 0);
        FT_SET_FINAL_FEAR_FACE(fighter, 40);
    }
    if is_excute(fighter) {
        if(PostureModule::lr(fighter.module_accessor)>=0.0){
            REQ_FINAL_START_CAMERA_arg3(fighter,Hash40::new("d04finalstart02.nuanmb"), false, false);
        }
        else{
            REQ_FINAL_START_CAMERA_arg3(fighter,Hash40::new("d04finalstart.nuanmb"), false, false);
        }
        //PostureModule::scale(fighter.module_accessor); //5.0
        //0x115660(-1239919212, 2.1);
        CAM_ZOOM_IN_arg5(fighter, 0.0, 0.0,0.0,0.0,0.0);
        FT_START_CUTIN(fighter);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        SlowModule::clear_whole(fighter.module_accessor);
        ArticleModule::remove_exist(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        CAM_ZOOM_OUT_FINAL(fighter);
    }
    frame(fighter.lua_state_agent, 30.0);
    let step = 4;
    /* 
    for i in (1..120).step_by(step)
    {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 80, 100, 80, 0, 1000.0, 0.0, 18.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, f32::NAN, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        }
        else
        {
            break;
        }
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        else
        {
            break;
        }
        wait(fighter.lua_state_agent, (step as f32)-1.0);
    }*/
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 80, 100, 80, 0, 1000.0, 0.0, 18.0, 30.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, f32::NAN, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}


#[acmd_script( agent = "richter", scripts = ["effect_finalstart","effect_finalairstart"] , category = ACMD_EFFECT )]
unsafe fn richter_final_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    if is_excute(fighter) {
        //EffectModule::req_screen(fighter.module_accessor,Hash40::new("bg_test2"),false,false,false);
    }
    frame(fighter.lua_state_agent, 90.0);
    if is_excute(fighter) {
        if (!WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_FINAL_HIT))
        {
            //EffectModule::remove_screen(fighter.module_accessor,Hash40::new("bg_richter_final"),-1);
        }
    }
}
#[acmd_script( agent = "richter", scripts = ["sound_finalstart","sound_finalairstart"] , category = ACMD_SOUND )]
unsafe fn richter_final_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_final01"));
    }
    frame(fighter.lua_state_agent, 55.0);
    {
        PLAY_SE(fighter, Hash40::new("vc_richter_final03"));
    }
}

#[acmd_script( agent = "richter", scripts = ["game_finalend","game_finalairend"] , category = ACMD_GAME )]
unsafe fn richter_final_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let step = 2;
    let mut size = 22.0;
    frame(fighter.lua_state_agent, 10.0);
    for i in (1..120).step_by(step)
    {
        if is_excute(fighter) {
            size = 12.0 + (i as f32)/1.0;
            ATTACK(fighter, 0, 1, Hash40::new("top"), 2.0, 80, 100, 1, 0, size, 0.0, 18.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, -1.0, 10, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            AttackModule::set_no_finish_camera(boma, 1, false, false);
            AttackModule::set_add_reaction_frame_revised(boma,1,10.0,false);
            AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        }
        wait(fighter.lua_state_agent, step as f32);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 90, 0, 40, size, 0.0, 18.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_attack_level(fighter.module_accessor, 0, (*FIGHTER_RYU_SAVING_LV_3).try_into().unwrap());
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)) {
            DamageModule::heal(boma,-25.0,0);
            //GetVar::set_int(boma, &mut vars::META_FRAME, vars::META_MAX*2);
            //GetVar::set_int(boma, &mut vars::META_ATTEMPTS, 0);
        }
    }
}
#[acmd_script( agent = "richter", scripts = ["effect_finalend","effect_finalairend"] , category = ACMD_EFFECT )]
unsafe fn richter_final_end_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    if is_excute(fighter) {
        //EffectModule::remove_screen(fighter.module_accessor,Hash40::new("bg_richter_final"),-1);
        EffectModule::req_screen(fighter.module_accessor,Hash40::new("bg_test2"),false,false,false);
    }
    frame(fighter.lua_state_agent, 129.0);
    if is_excute(fighter) {
        EffectModule::remove_screen(fighter.module_accessor,Hash40::new("bg_test2"),-1);
    }
}
#[acmd_script( agent = "richter", scripts = ["sound_finalend","sound_finalairend"] , category = ACMD_SOUND )]
unsafe fn richter_final_end_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(fighter.lua_state_agent, 121.0);
    if is_excute(fighter) {
        if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)) {
            PLAY_SE(fighter, Hash40::new("vc_richter_final02"));
        }
        else
        {
            PLAY_SE(fighter, Hash40::new("vc_richter_final03"));
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_final_start_game,
        richter_final_start_effect,
        richter_final_start_sound,
        richter_final_end_game,
        richter_final_end_effect,
        richter_final_end_sound
    );
}
