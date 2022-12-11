use super::*;

#[acmd_script( agent = "richter_axe", script = "game_fly" , category = ACMD_GAME )]
unsafe fn axe_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("axe"), 3.5, 82, 15, 0, 55, 6.0, 0.0, -3.0, -1.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.1);
    }
    /*
    wait(lua_state, 80.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("axe"), 5.0, 361, kbg, 0, bkb, 3.0, 0.0, -3.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    */
}
#[acmd_script( agent = "richter_axe", script = "effect_fly" , category = ACMD_EFFECT )]
unsafe fn axe_fly_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    for x in 1..30 {
        if is_excute(fighter) {
            //sys_flame
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 6, 0, 0, -90, 0, 1, true);
        }
        else
        {
            break;
        }
        wait(lua_state, 8.0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        axe_fly_game,
        axe_fly_effect
    );
}

