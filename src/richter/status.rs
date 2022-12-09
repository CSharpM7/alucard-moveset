use super::*;

pub const PREV_STATUS_KIND:      i32 = 0xA;
pub const STATUS_KIND:           i32 = 0xB;
pub const SUB_STATUS:            i32 = 0x15;
pub const SITUATION_KIND:        i32 = 0x16;

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn catch_pull_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_S {
        *SITUATION_KIND_NONE
    }
    else {
        *SITUATION_KIND_GROUND
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(situation),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        true,
        false,
        0,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT |
            *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE
        ) as u32,
        0,
        0
    );
    0.into()
}

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn catch_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_S {
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION
        );
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
    }
    fighter.status_CatchPull_common(hash40("catch_wait").into());
    fighter.main_shift(catch_pull_main_loop)
}

unsafe extern "C" fn catch_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_S
    && CancelModule::is_enable_cancel(fighter.module_accessor) {
        let status = fighter.global_table[0x45].get_i32();
        fighter.change_status(status.into(), false.into());
        1.into()
    }
    else {
        fighter.status_CatchPull_Main()
    }
}

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion;
    let kinetic;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        motion = Hash40::new("special_air_s1");
        kinetic = *FIGHTER_KINETIC_TYPE_FALL;
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    }
    else {
        motion = Hash40::new("special_s1");
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    }
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    MotionModule::change_motion(
        fighter.module_accessor,
        motion,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    // <HDR>
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s_substatus as *const () as _));
    // </HDR>
    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && param_1.get_bool() {

        // enable fastfall
        if fighter.is_cat_flag(Cat2::FallJump)
            && fighter.stick_y() < -0.66
            && KineticModule::get_sum_speed_y(fighter.boma(), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
            WorkModule::set_flag(fighter.boma(), true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
    0.into()
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_air = MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_s1");
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if is_air {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_transition_group_check_air_landing().get_bool()
            || fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let status = if situation != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
        };
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if is_air {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
        };
        fighter.change_status(status.into(), false.into());
    }
    1.into()
}
/*
#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn richter_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    return false.into();
}

#[status_script(agent = "richter", status = FIGHTER_RICHTER_STATUS_KIND_SPECIAL_S_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn richter_special_s_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    return original!(fighter);
}
*/

pub fn install() {
    install_status_scripts!(
        special_s_main,
        catch_pull_main
    );
}
