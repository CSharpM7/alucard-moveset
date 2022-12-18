use super::*;

#[acmd_script( agent = "richter_whip", script = "game_attackhold" , category = ACMD_GAME )]
unsafe fn whip_attack_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("whip"), 1.0, 80, 35, 0, 40, 2.2, 0.0, 0.0, 0.0, Some(0.0), Some(vars::LENGTH), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 13, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
}

pub fn install() {
    install_acmd_scripts!(
        whip_attack_hold_game
    );
}
