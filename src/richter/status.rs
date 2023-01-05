use super::*;

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn richter_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    WorkModule::set_int64(fighter.module_accessor, 0xa28f17495 as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, 0xed8a31e01 as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND_AIR);
    //let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
    let lr_stick_x = 0.0;
    WorkModule::set_float(fighter.module_accessor, lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    //let dir_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_stick_x"));
    let dir_stick_x = 0.0;
    WorkModule::set_float(fighter.module_accessor, dir_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
    let dir_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_mul"));
    WorkModule::set_float(fighter.module_accessor, dir_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
    //let accel_y_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("accel_y_air"));
    let accel_y_air = 0.0;
    WorkModule::set_float(fighter.module_accessor, accel_y_air, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
    let pass_mul_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("pass_mul_ground"));
    WorkModule::set_float(fighter.module_accessor, pass_mul_ground, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
    let pass_mul_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("pass_mul_air"));
    WorkModule::set_float(fighter.module_accessor, pass_mul_air, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
    let fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul"));
    WorkModule::set_float(fighter.module_accessor, fall_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);

    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_int(fighter.module_accessor, landing_frame, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);

    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_X_MUL);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_Y_MUL);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    let main_sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    fighter.super_jump_punch(L2CValue::Void());
    fighter.sub_shift_status_main(L2CValue::Ptr(richter_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn richter_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_curse"), Hash40::new("top"), -5, 7.5, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(fighter,1,0,1);
        return 0.into();
    }

    let currentFrame = fighter.motion_frame();
    if MotionModule::is_end(fighter.module_accessor)
    || currentFrame > upspecial::FRAME_END {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN)
    {
        let article_boma = get_article_boma(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN);
        //snap_article(fighter.module_accessor,*FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN,Hash40::new("trans"),Hash40::new("bat_trans"));
        
        let mut pos = Vector3f::zero();
        let offset = ModelModule::joint_global_offset_from_top(boma, Hash40{hash: hash40("trans")}, &mut pos);        
        let newPos = Vector3f{x: PostureModule::pos_x(boma) + pos.x, y: PostureModule::pos_y(boma) + pos.y + 0.0, z: PostureModule::pos_z(boma) + pos.z};
        //ModelModule::set_joint_translate(article_boma, Hash40::new("trans"), &newPos, true,false);
        PostureModule::set_pos(article_boma,  &newPos);
        //ArticleModule::set_pos(article_boma,  *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN,newPos);
        //PostureModule::init_pos(article_boma,  &newPos,true,true);
        
        let degree = GetVar::get_float(fighter.module_accessor, &mut vars::BAT_DEGREE);
        let lr = if (PostureModule::lr(fighter.module_accessor) == PostureModule::lr(article_boma)) {0.0} else {180.0};
        PostureModule:: set_rot(
            article_boma,
            &Vector3f{x: -degree, y: lr, z: 0.0},
            0
        ); 
    }
    
    if WorkModule::is_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE) {        
        if GroundModule::can_entry_cliff(fighter.module_accessor) == 1
        && ControlModule::get_stick_y(fighter.module_accessor) > -0.5 {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE),
                L2CValue::Bool(false)
            );
            return 1.into()
        }

        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_deathscythe_trace"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 0.75, true, 0, 0, 0, 0, 0, false, false);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            //fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            fighter.change_status(FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32_LANDING.into(), false.into());
            return 1.into();
        }
    }
    
    return 0.into()
}


#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn richter_special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    return 0.into()
}
#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe extern "C" fn richter_special_hi_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    
    WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE_CONTROL);
    PostureModule:: set_rot(
        fighter.module_accessor,
        &Vector3f{x: 0.0, y: 0.0, z: 0.0},
        0
    ); 
    return 0.into()
}
#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_CLIFF_CATCH, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn richter_cliff_catch_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    return original!(fighter);
}

pub const CHECK_SPECIAL_S_UNIQ:            i32 = 0x39;
unsafe extern "C" fn richter_special_s_preU(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (!GetVar::get_bool(fighter.module_accessor, &mut vars::SPECIAL_S_AERIAL))
    {
        return 0.into();
    }
    println!("?!?");
    GetVar::set_bool(fighter.module_accessor, &mut vars::SPECIAL_S_AERIAL, false);
    let entry = get_entry(fighter);
    let boma = fighter.module_accessor;
    GetVar::set_int(fighter.module_accessor, &mut vars::DIVE_TARGET, 0);


    return 1.into();
}

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn richter_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{

    if (!GetVar::get_bool(fighter.module_accessor, &mut vars::SPECIAL_S_AERIAL))
    {
        return 0.into();
    }
    println!("?!?");
    GetVar::set_bool(fighter.module_accessor, &mut vars::SPECIAL_S_AERIAL, false);
    let entry = get_entry(fighter);
    let boma = fighter.module_accessor;
    GetVar::set_int(fighter.module_accessor, &mut vars::DIVE_TARGET, 0);


    return 1.into();
}

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn richter_special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_ROBBED);
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.5
    );    
    return false.into();
}
#[status_script(agent = "richter", status = FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn richter_special_s2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE,
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        true,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0,
    );
    L2CValue::I32(0)
}
#[status_script(agent = "richter", status = FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn richter_special_s2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry = get_entry(fighter);
    let boma = fighter.module_accessor;

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
      );
      sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY
      );
      sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0
      );
    let mut defenderTest = get_fighter_common_from_entry_id(GetVar::get_int(boma, &mut vars::DIVE_TARGET) as u32);
    if (defenderTest.is_none()){
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return 0.into();
    }
    let defender = defenderTest.unwrap();
    if (defender.is_status(*FIGHTER_STATUS_KIND_DEAD)){
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return 0.into();
    }
/* 
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(), false.into());
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
        defender.change_status(FIGHTER_STATUS_KIND_DEAD.into(), false.into());
    }
*/
    return 0.into()
}

#[status_script(agent = "richter", status = FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe extern "C" fn richter_special_s2_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry = get_entry(fighter);
    let boma = fighter.module_accessor;

    if (GetVar::get_int(boma, &mut vars::DIVE_TARGET) > 0)
    {
        let defender = get_fighter_common_from_entry_id(GetVar::get_int(boma, &mut vars::DIVE_TARGET) as u32);
        if (!defender.is_none()){
            (defender.unwrap()).change_status(FIGHTER_STATUS_KIND_CAPTURE_CUT.into(), false.into());
        }
        GetVar::set_int(boma, &mut vars::DIVE_TARGET, 0);
    }

    return 0.into()
}


#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn richter_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry = get_entry(fighter);
    let boma = fighter.module_accessor;
    GetVar::set_int(boma, &mut vars::SPECIAL_N_SPAWN,0);

    return original!(fighter)
}

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn richter_special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry = get_entry(fighter);
    let boma = fighter.module_accessor;
    let currentFrame = MotionModule::frame(fighter.module_accessor);
    let spawnFrame = neutralspecial::SPAWN_FRAME;

    if currentFrame >= neutralspecial::CHECK_FRAME && currentFrame<spawnFrame-1.0 {
        if currentFrame >= spawnFrame-3.0 {
            let spawnType = if !ArticleModule::is_exist(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_AXE)
            {neutralspecial::SPAWN_TYPE_INFERNO} else {-1};
            GetVar::set_int(boma, &mut vars::SPECIAL_N_SPAWN,spawnType);
            return 0.into()
        }
        else if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL)
        || !(vars::meta_is_active(boma)) 
        {
            let spawnType = if !ArticleModule::is_exist(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_CROSS)
            {neutralspecial::SPAWN_TYPE_HELLFIRE} else {-1};
            GetVar::set_int(boma, &mut vars::SPECIAL_N_SPAWN,spawnType);
        }
        else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
        && ArticleModule::is_exist(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_AXE)
        {
            GetVar::set_int(boma, &mut vars::SPECIAL_N_SPAWN,-1);
        }

        if (GetVar::get_int(boma, &mut vars::SPECIAL_N_SPAWN) != 0)
        {
            STOP_SE(fighter, Hash40::new("se_richter_special_s01"));
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_sscope_hold"),true,false);
            MotionModule::set_frame_sync_anim_cmd(boma, spawnFrame-1.0, true,false,false);
            return 0.into()
        }
    }
    else if currentFrame < neutralspecial::CHECK_FRAME
    {
        GetVar::set_int(boma, &mut vars::SPECIAL_N_SPAWN,0);
    }

    return 0.into()
}

#[status_script(agent = "richter", status = FIGHTER_SIMON_STATUS_KIND_FINAL_READY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn richter_final_ready_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_SIMON_STATUS_KIND_FINAL_END.into(), false.into());
    return true.into();
}
#[status_script(agent = "richter", status = FIGHTER_SIMON_STATUS_KIND_FINAL_READY, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn richter_final_ready_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let currentFrame = MotionModule::frame(fighter.module_accessor);
    if MotionModule::is_end(fighter.module_accessor) 
    || currentFrame > 31.0 {
        AttackModule::clear_all(fighter.module_accessor);
        fighter.change_status(FIGHTER_SIMON_STATUS_KIND_FINAL_END.into(), false.into());
        return true.into();
    }
    return false.into();
}

#[status_script(agent = "richter", status = FIGHTER_SIMON_STATUS_KIND_FINAL_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn richter_final_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    return false.into();
}


#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_RICHTER {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(richter_special_s_preU as *const () as _));
    }
}

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_ENTRY, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn richter_entry_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN)
    {
        let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN);
        
        let mut pos = Vector3f::zero();
        let offset = ModelModule::joint_global_offset_from_top(fighter.module_accessor, Hash40{hash: hash40("hip")}, &mut pos);        
        let newPos = Vector3f{x: PostureModule::pos_x(fighter.module_accessor) + pos.x, y: PostureModule::pos_y(fighter.module_accessor) + pos.y + 0.0, z: PostureModule::pos_z(fighter.module_accessor) + pos.z};
        PostureModule::set_pos(article_boma,  &newPos);

        let lr = if (PostureModule::lr(fighter.module_accessor) == PostureModule::lr(article_boma)) {0.0} else {-90.0};
        PostureModule:: set_rot(
            article_boma,
            &Vector3f{x: 0.0, y: lr, z: 0.0},
            0
        ); 

        if MotionModule::is_end(fighter.module_accessor) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    return false.into();
}

pub fn install() {
    install_status_scripts!(
        richter_special_n_pre,
        richter_special_n_exec,
        richter_special_hi_main,
        richter_special_hi_exec,
        richter_special_hi_exit,
        //richter_special_s_pre,
        richter_special_s_exec,
        richter_special_s2_pre,
        richter_special_s2_exec,
        richter_special_s2_exit,
        //richter_cliff_catch_exec,
        richter_final_end_pre,
        //richter_final_ready_pre
        richter_final_ready_exec,
        richter_entry_exec
    );
    
    install_agent_init_callback!(
        agent_init
    );
}