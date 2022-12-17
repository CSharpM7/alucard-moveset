
use super::*;

#[acmd_script( agent = "richter", script = "game_specialairs1" , category = ACMD_GAME )]
unsafe fn richter_special_sa_game(fighter: &mut L2CAgentBase) {
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
        CATCH(fighter, 0, Hash40::new("top"), 12.0, 0.0, 14.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *COLLISION_SITUATION_MASK_GA);

        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);

        //AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("rot"), &offset, 0, false);
        
    }
    wait(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
        AttackModule::clear_all(boma);
    }
}

//#[acmd_script( agent = "richter", scripts = ["game_specials1","game_specialairs1"] , category = ACMD_GAME )]
#[acmd_script( agent = "richter", script = "game_specials1" , category = ACMD_GAME )]
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

        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.0, 0.0, 6.6, 6.0, Some(0.0), Some(6.6), Some(21.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, -1.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);

        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 5.0, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(11.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, -1.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);

        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);

        //AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("rot"), &offset, 0, false);
        
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
    let defender_boma = get_boma_from_entry_id(opff::get_dive_target(entry));
    
    let offset = smash::phx::Vector2f { x: 0.0, y: -8.0 };
    if is_excute(fighter) {

        for i in 0..11{
            HIT_NO(fighter, i as u64, *HIT_STATUS_XLU);
        }

        let i = 0;
        let offset = Vector2f { x: 0.0, y: -8.0 };
        let size = (7.0-(i as f32)/2.0).max(2.0);
        
        //ATTACK(fighter, 0, 0, Hash40::new("throw"), 0.1, 368, 100, 0, 0, size, 0.0, 0.0, 0.0, None,None,None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, -1.0, 1, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        
        //ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.1, 368, 0, 0, 0, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 60, 50, 0, 90, 1.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);

        AttackModule::set_catch_only_all(fighter.module_accessor, true,false);

        AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("throw"), &offset, 0, false);
        AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        AttackModule::set_no_uniq_effect_all(fighter.module_accessor, true, false);
        AttackModule::set_no_finish_camera_ex(fighter.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera_ex(fighter.module_accessor, 0, true, false);

    }
    /*
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        //ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), (*defender_boma).battle_object_id as u64, WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        //(*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_THROWN, false);
        CaptureModule::thrown(&mut *defender_boma);
    }
    */
    for i in 1..27{
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            //ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), (*defender_boma).battle_object_id as u64, WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            let mut pos = Vector3f::zero();
            let offset = ModelModule::joint_global_offset_from_top(boma, Hash40{hash: hash40("throw")}, &mut pos);        
            let newPos = Vector3f{x: PostureModule::pos_x(boma) + pos.x, y: PostureModule::pos_y(boma) + pos.y + 0.0, z: PostureModule::pos_z(boma) + pos.z}; //Set by the attacker

            GroundModule::set_attach_ground(defender_boma, false);
            KineticModule::change_kinetic(&mut *defender_boma, *FIGHTER_KINETIC_TYPE_CAPTURE);
            PostureModule::set_pos(defender_boma, &newPos);
            //ModelModule::set_joint_translate(defender_boma, Hash40::new("hip"), &newPos, true,false);
        }
        else
        {
            (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_CAPTURE_CUT, false);
        }
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 60, 50, 0, 90, 1.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_CAPTURE_WAIT, false);
        
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), (*defender_boma).battle_object_id as u64, WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));

        if (AttackModule::is_hit_abs(boma)){
            let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            opff::meta_start(entry);
        }
        //opff::meta_start(entry);
        if CatchModule::is_catch(&mut *defender_boma){
            CaptureModule::thrown(&mut *defender_boma);
            CatchModule::set_send_cut_event(defender_boma,true);
        }
    }
    wait(lua_state, 1.0);
    {
        opff::set_dive_target(entry, 0);
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

#[acmd_script( agent = "richter", scripts = ["sound_specials1","sound_specialairs1"] , category = ACMD_SOUND )]
unsafe fn richter_special_s1_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script( agent = "richter", scripts = ["sound_specials2","sound_specialairs2"] , category = ACMD_SOUND )]
unsafe fn richter_special_s2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}

pub fn install() {
    install_acmd_scripts!(
        richter_special_s_game,
        richter_special_sa_game,

        richter_special_s2_game,
        richter_special_s2_effect,

        richter_special_s1_sound,
        richter_special_s2_sound
    );
}
