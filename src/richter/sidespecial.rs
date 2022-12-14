
use super::*;

#[acmd_script( agent = "richter", script = "game_specialairs1" , category = ACMD_GAME )]
unsafe fn richter_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    frame(lua_state, 10.0);
    if is_excute(fighter) {
        /*
        CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 6.6, 2.35, Some(0.0), Some(6.6), Some(10.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        macros::game_CaptureCutCommon(fighter);*/
        CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(18.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 6.6, 2.35, Some(0.0), Some(6.6), Some(16.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        macros::game_CaptureCutCommon(fighter);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

#[acmd_script( agent = "richter", script = "game_specials1" , category = ACMD_GAME )]
unsafe fn richter_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 10.0);
    if is_excute(fighter) {

        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
        let offset = smash::phx::Vector2f { x: 8.0, y: -6.6 };

        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 368, 100, 0, 0, 2.6, 0.0, 6.6, 8.0, Some(0.0), Some(6.6), Some(13.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, -1.0, 1, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);

        AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("rot"), &offset, 0, false);
        
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
        AttackModule::clear_all(boma);
    }
}


#[acmd_script( agent = "richter", script = "game_specials2" , category = ACMD_GAME )]
unsafe fn richter_special_s2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let target = opff::get_dive_target(entry);
    
    if is_excute(fighter) {
        /*
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 38.0, 74, 112, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 30.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);

        */
        for i in 0..11{
            HIT_NO(fighter, i as u64, *HIT_STATUS_XLU);
        }

        //for i in 0..11{
            let i = 0;
            let offset = smash::phx::Vector2f { x: 8.0, y: -6.6 };
            let size = (7.0-(i as f32)/2.0).max(2.0);
            
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 368, 100, 0, 0, size, 0.0, 10.6, 8.0, None,None,None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, -1.0, 1, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);

            AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
            AttackModule::set_no_uniq_effect_all(fighter.module_accessor, true, false);
            AttackModule::set_no_finish_camera_ex(fighter.module_accessor, 0, true, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);

            let lerp = 0;
            AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("rot"), &offset, lerp, false);

            //wait(lua_state, 2.0);
            //AttackModule::clear_all(boma);
            //wait(lua_state, 1.0);
        //}

    }
    wait(lua_state,2.0);
    if is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor, /*ID*/ 0, /*Size*/ 2.0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_special_s_game,
        richter_special_air_s_game,

        richter_special_s2_game,
    );
}
