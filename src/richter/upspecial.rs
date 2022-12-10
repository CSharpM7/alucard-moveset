
use super::*;

#[acmd_script( agent = "richter", script = "game_specialn" , category = ACMD_GAME )]
unsafe fn richter_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
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
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        for i in 0..10{
            if i != 2{
            HIT_NO(fighter, i, *HIT_STATUS_OFF);
            }
        }
        GroundModule::attach_ground(boma, false);
        let mut stick_x: f32 = ControlModule::get_stick_x(boma);
        let mut stick_y: f32 = ControlModule::get_stick_y(boma);
        if (stick_x.abs() < 0.1 && stick_y.abs() < 0.1)
        {
            stick_x = 1.0;
            stick_y = 0.0;
        }
        let normalized = sv_math::vec2_normalize(stick_x, stick_y);
        let lr = PostureModule::lr(boma);
        let motion_factor = 1.0;

        //ControlModule::get_stick_angle(module_accessor)
        //SET_SPEED_EX(fighter,stick_x*lr*motion_factor,stick_y*motion_factor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }


    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 95, 100, 155, 0, 8.5, 0.0, 9.5, 10.5, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 75.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
        for i in 0..10{
            HIT_NO(fighter, i, *HIT_STATUS_NORMAL);
        }
    }
}
#[acmd_script( agent = "richter", script = "game_specialairhi" , category = ACMD_GAME )]
unsafe fn richter_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 20.0);
    if is_excute(fighter) {
        for i in 0..10{
            if i != 2{
                HIT_NO(fighter, i, *HIT_STATUS_OFF);
            }
        }
    }
}

#[acmd_script( agent = "richter", script = "effect_specialhi" , category = ACMD_EFFECT )]
unsafe fn richter_special_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_final_coffin_start"), Hash40::new("hip"), 0,0,0,0,0,0, 0.625, true);
        LAST_EFFECT_SET_RATE(fighter,1.75);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace_smash"), Hash40::new("top"), 5, 7.5, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace_smash"), Hash40::new("top"), 5, 15, 0, 0, 0, 0, 1.5, true);
    }
}
#[acmd_script( agent = "richter", script = "effect_specialairhi" , category = ACMD_EFFECT )]
unsafe fn richter_special_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_final_coffin_start"), Hash40::new("hip"), 0,0,0,0,0,0, 0.625, true);
        LAST_EFFECT_SET_RATE(fighter,3.0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace_smash"), Hash40::new("top"), 5, 7.5, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace_smash"), Hash40::new("top"), 5, 15, 0, 0, 0, 0, 1.5, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_special_hi_game,
        richter_special_air_hi_game,

        richter_special_hi_effect,
        richter_special_air_hi_effect
    );
}
