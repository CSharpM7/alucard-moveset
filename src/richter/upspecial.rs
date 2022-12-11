
use super::*;

const FRAME_ATTACK: f32 = 39.0;
const FRAME_END: f32 = 100.0;

#[acmd_script( agent = "richter", script = "game_specialhi" , category = ACMD_GAME )]
unsafe fn richter_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        /*
        let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)+3.0, z: 0.0 };
        PostureModule::set_pos(fighter.module_accessor, &pos);
        PostureModule::init_pos(fighter.module_accessor, &pos, false, false);
        */
    }
    frame(lua_state, FRAME_ATTACK);
    if is_excute(fighter) {
        for i in 0..10{
            if i != 2{
            HIT_NO(fighter, i, *HIT_STATUS_OFF);
            }
        }
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES_NEAR);
        /*
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
        */
        //ControlModule::get_stick_angle(module_accessor)
        //SET_SPEED_EX(fighter,stick_x*lr*motion_factor,stick_y*motion_factor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    frame(lua_state, FRAME_END);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
        for i in 0..10{
            HIT_NO(fighter, i, *HIT_STATUS_NORMAL);
        }
    }
    FT_MOTION_RATE(fighter,0.01);
}
#[acmd_script( agent = "richter", script = "game_specialairhi" , category = ACMD_GAME )]
unsafe fn richter_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, FRAME_ATTACK);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES_NEAR);

        for i in 0..10{
            if i != 2{
                HIT_NO(fighter, i, *HIT_STATUS_OFF);
            }
        }
    }
    frame(lua_state, FRAME_END);
    if is_excute(fighter) {
        for i in 0..10{
            HIT_NO(fighter, i, *HIT_STATUS_NORMAL);
        }
    }
    FT_MOTION_RATE(fighter,0.0);
}

#[acmd_script( agent = "richter", script = "effect_specialhi" , category = ACMD_EFFECT )]
unsafe fn richter_special_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_final_coffin_start"), Hash40::new("hip"), 0,0,0,0,0,0, 0.625, true);
        LAST_EFFECT_SET_RATE(fighter,3.0);
        LAST_EFFECT_SET_COLOR(fighter,1,0,4);
    }
    frame(lua_state, FRAME_ATTACK-2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_curse"), Hash40::new("top"), -5, 7.5, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(fighter,1,0,1);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.5, true);
    }
}
#[acmd_script( agent = "richter", script = "effect_specialairhi" , category = ACMD_EFFECT )]
unsafe fn richter_special_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter,0.5);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter,1);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_final_coffin_start"), Hash40::new("hip"), 0,0,0,0,0,0, 0.625, true);
        LAST_EFFECT_SET_RATE(fighter,3.0);
        LAST_EFFECT_SET_COLOR(fighter,1,0,4);
    }
    frame(lua_state, FRAME_ATTACK-2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_curse"), Hash40::new("top"), -5, 7.5, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(fighter,1,0,1);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.5, true);
    }
}


#[acmd_script( agent = "richter", script = "game_attacklw32landing" , category = ACMD_GAME )]
unsafe fn richter_special_hi_landing_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter,2.0);
}

pub fn install() {
    install_acmd_scripts!(
        richter_special_hi_game,
        richter_special_air_hi_game,
        richter_special_hi_landing_game,

        richter_special_hi_effect,
        richter_special_air_hi_effect
    );
}
