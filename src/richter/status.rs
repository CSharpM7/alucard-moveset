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
    fighter.sub_shift_status_main(L2CValue::Ptr(richter_specialhi_main_loop as *const () as _))
}

unsafe extern "C" fn richter_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_curse"), Hash40::new("top"), -5, 7.5, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(fighter,1,0,1);
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) == true 
    || WorkModule::is_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE_CONTROL) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    
    let currentFrame = fighter.motion_frame();
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

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn richter_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    opff::set_dive_target(entry, 0);


    return original!(fighter);
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
    /*
    let in_Hitstop = SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 ;
    let has_hit = AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || in_Hitstop;
    if (has_hit)
    {
        fighter.change_status(FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2.into(), true.into());
        return false.into();
    }
    */

    
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
      if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(*FIGHTER_STATUS_KIND_DEAD, false);
      }
    return 0.into()
}

pub fn install() {
    install_status_scripts!(
        richter_special_hi_main,
        richter_special_s_pre,
        richter_special_s_exec,
        richter_special_s2_pre,
        richter_special_s2_exec
    );
    
}