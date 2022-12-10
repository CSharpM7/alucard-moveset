use super::*;

#[acmd_script( agent = "richter_whip", script = "game_attackairb" , category = ACMD_GAME )]
unsafe fn whip_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 1);
    frame(lua_state, 26.0);
}
#[acmd_script( agent = "richter_whip", script = "effect_attackairb" , category = ACMD_EFFECT )]
unsafe fn whip_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("hookshot7"), -8, 0, 0, 180, -60, -90, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot9"), 0, 0, 0, 0, 180, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_whip_straight"), true, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        whip_attack_air_b_game,
        whip_attack_air_b_effect
    );
}

