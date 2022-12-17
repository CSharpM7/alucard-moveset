use super::*;

#[acmd_script( agent = "richter_cross", script = "game_fly" , category = ACMD_GAME )]
unsafe fn cross_fly_game(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    if is_excute(weapon) {
        ATTACK(weapon, 0, 0, Hash40::new("cross"), 9.0, 361, 50, 0, 35, 2.0, 0.0, 6.0, 0.0, Some(0.0),Some(-6.0),Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
}
#[acmd_script( agent = "richter_cross", script = "effect_fly" , category = ACMD_EFFECT )]
unsafe fn cross_fly_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let maxFrame =  WorkModule::get_param_int(weapon.module_accessor, hash40("param_cross"), hash40("turn_frame")) as f32;
    let maxFrameX = (maxFrame) as i32;
    for x in (1..maxFrameX).step_by(5) {
        if is_excute(weapon) {
            //sys_flame
            let ys = vec![-5, 0, 5]; 
            for y in ys.into_iter()
            {
                EFFECT_FOLLOW_NO_STOP(weapon, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, y, 0, 0, -90, 0, 0.3, true);
            }
        }
        else
        {
            break;
        }
        wait(lua_state, 5.0);
    }
}
#[acmd_script( agent = "richter_cross", script = "sound_fly" , category = ACMD_SOUND )]
unsafe fn cross_fly_sound(weapon: &mut L2CAgentBase) {
    if is_excute(weapon) {
        PLAY_STATUS(weapon, Hash40::new("se_richter_special_s01"));
    }
}

#[acmd_script( agent = "richter_cross", script = "game_turn" , category = ACMD_GAME )]
unsafe fn cross_turn_game(weapon: &mut L2CAgentBase) {
}
#[acmd_script( agent = "richter_cross", script = "effect_turn" , category = ACMD_EFFECT )]
unsafe fn cross_turn_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let ys = vec![-5, 0, 5]; 
    for y in ys.into_iter()
    {
        EFFECT(weapon, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, y, 0, 0, 0, 0, 1.0, 0,0,0,0,0,0,true);
    }
}
#[acmd_script( agent = "richter_cross", script = "sound_turn" , category = ACMD_SOUND )]
unsafe fn cross_turn_sound(weapon: &mut L2CAgentBase) {
    if is_excute(weapon) {
        PLAY_SE_REMAIN(weapon, Hash40::new("se_richter_special_s02_smash"));
    }
}


pub fn install() {
    install_acmd_scripts!(
        cross_fly_game,
        cross_fly_effect,
        cross_fly_sound,

        cross_turn_game,
        cross_turn_effect,
        cross_turn_sound
    );
}

