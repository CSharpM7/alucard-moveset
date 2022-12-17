use super::*;

#[acmd_script( agent = "richter_axe", script = "game_fly" , category = ACMD_GAME )]
unsafe fn axe_fly_game(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    if is_excute(weapon) {
        ATTACK(weapon, 0, 0, Hash40::new("axe"), 3.5, 60, 15, 0, 35, 5.0, 0.0, -1.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_MAGIC);
        
        ATK_SET_SHIELD_SETOFF_MUL(weapon, 0, 1.25);
    }
}
#[acmd_script( agent = "richter_axe", script = "effect_fly" , category = ACMD_EFFECT )]
unsafe fn axe_fly_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    for x in 1..30 {
        if is_excute(weapon) {
            //sys_flame
            EFFECT_FOLLOW_NO_STOP(weapon, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 6, 0, 0, -90, 0, 1, true);
        }
        else
        {
            break;
        }
        wait(lua_state, 8.0);
    }
}

#[acmd_script( agent = "richter_axe", script = "sound_fly" , category = ACMD_SOUND )]
unsafe fn axe_fly_sound(weapon: &mut L2CAgentBase) {
    if is_excute(weapon) {
        PLAY_STATUS(weapon, Hash40::new("se_richter_special_s02"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        axe_fly_game,
        axe_fly_effect,
        axe_fly_sound
    );
}

