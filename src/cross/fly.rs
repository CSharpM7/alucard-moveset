use super::*;

#[acmd_script( agent = "richter_cross", script = "game_fly" , category = ACMD_GAME )]
unsafe fn cross_fly_game(fighter: &mut L2CAgentBase) {
    let kbg = 50;
    let bkb = 75;
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("cross"), 10.0, 361, kbg, 0, bkb, 2.0, 0.0, 6.0, 0.0, Some(0.0),Some(-6.0),Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
}
#[acmd_script( agent = "richter_cross", script = "effect_fly" , category = ACMD_EFFECT )]
unsafe fn cross_fly_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    for x in 1..30 {
        if is_excute(fighter) {
            //sys_flame
            let ys = vec![-5, 0, 5]; 
            for y in ys.into_iter()
            {
                EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, y, 0, 0, -90, 0, 0.3, true);
            }
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
        cross_fly_game,
        cross_fly_effect
    );
}

