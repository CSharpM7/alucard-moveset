
use super::*;

#[acmd_script( agent = "richter", script = "game_specialairs1" , category = ACMD_GAME )]
unsafe fn richter_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    frame(lua_state, 14.0);
    if is_excute(fighter) {
        /*
        CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 6.6, 2.35, Some(0.0), Some(6.6), Some(10.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        macros::game_CaptureCutCommon(fighter);*/
        CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 8.6, 4.0, Some(0.0), Some(8.6), Some(18.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 8.6, 2.35, Some(0.0), Some(8.6), Some(16.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        macros::game_CaptureCutCommon(fighter);
    }
    wait(lua_state, 16.0);
    if is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_specials1","game_specialairs1"] , category = ACMD_GAME )]
unsafe fn richter_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.5
        );
        let speed_y = KineticModule::get_sum_speed_y(boma, 1);
        println!("{}",speed_y);
        if (speed_y < 0.0)
        {
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY
        );
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {

        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
        let offset = smash::phx::Vector2f { x: 8.0, y: -6.6 };

        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 368, 100, 0, 0, 3.0, 0.0, 6.6, 6.0, Some(0.0), Some(6.6), Some(11.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, -1.0, 1, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);

        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 368, 100, 0, 0, 5.0, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(11.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, -1.0, 1, false, false, false, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);

        AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("rot"), &offset, 0, false);
        
    }
    wait(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
        AttackModule::clear_all(boma);
    }
}


#[acmd_script( agent = "richter", scripts = ["game_specials2","game_specialairs2"] , category = ACMD_GAME )]
unsafe fn richter_special_s2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let target = opff::get_dive_target(entry);
    
    if is_excute(fighter) {
        for i in 0..11{
            HIT_NO(fighter, i as u64, *HIT_STATUS_XLU);
        }

        let i = 0;
        let offset = smash::phx::Vector2f { x: 0.0, y: -8.0 };
        let size = (7.0-(i as f32)/2.0).max(2.0);
        
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 0.1, 368, 100, 0, 0, size, 0.0, 0.0, 0.0, None,None,None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, -1.0, 1, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);

        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        AttackModule::set_no_uniq_effect_all(fighter.module_accessor, true, false);
        AttackModule::set_no_finish_camera_ex(fighter.module_accessor, 0, true, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);

        let lerp = 0;
        AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("throw"), &offset, lerp, false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        //deathball?
        //
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 10.0, 60, 50, 0, 90, 7.0, 0.0, 0.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 1.0);
    {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)){
            let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            opff::meta_start(entry);
        }
        AttackModule::clear_all(boma);
        for i in 0..11{
            HIT_NO(fighter, i as u64, *HIT_STATUS_XLU);
        }
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter", scripts = ["effect_specials2","effect_specialairs2"] , category = ACMD_EFFECT )]
unsafe fn richter_special_s2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        //EFFECT_FOLLOW(fighter, Hash40::new("sys_sscope_bullet"), Hash40::new("throw"), 0,0,0,0,0,0, 4.0, true);
        //LAST_EFFECT_SET_COLOR(fighter,1,0.1,0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_final_coffin_vacuum"), Hash40::new("throw"), 0,0,0,0,0,0, 0.625, true);
        LAST_EFFECT_SET_RATE(fighter,4.5);
        LAST_EFFECT_SET_COLOR(fighter,1,0,4);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_sscope_bullet"), false,true);
        EFFECT(fighter, Hash40::new("sys_bomb_c"), Hash40::new("throw"), 0,0,0, 0, 0, 0, 0.8, 0,0,0,0,0,0,true);
    }
}


pub fn install() {
    install_acmd_scripts!(
        richter_special_s_game,
//        richter_special_air_s_game,

        richter_special_s2_game,
        richter_special_s2_effect
    );
}
