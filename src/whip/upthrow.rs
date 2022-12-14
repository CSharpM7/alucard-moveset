use super::*;

#[acmd_script( agent = "richter_whip", script = "game_throwhi" , category = ACMD_GAME )]
unsafe fn whip_throw_hi_game(weapon: &mut L2CAgentBase) {

}
#[acmd_script( agent = "richter_whip", script = "effect_throwhi" , category = ACMD_EFFECT )]
unsafe fn whip_throw_hi_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.module_accessor;
    
    frame(lua_state, 12.0);
    if is_excute(weapon) {
        EFFECT_FOLLOW(weapon, Hash40::new("sys_blackball_set"), Hash40::new("top"), 0, 2.0, -7.0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(weapon,2.0);
        //LAST_EFFECT_SET_COLOR(weapon,1,0.125,1);
    }
    frame(lua_state, 20.0);
    if is_excute(weapon) {
        EFFECT_FOLLOW(weapon, Hash40::new("sys_greenshell_trace"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(weapon,1,0.125,1);
        EFFECT_FOLLOW(weapon, Hash40::new("sys_deathscythe_shadow"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 0.5, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        whip_throw_hi_game,
        whip_throw_hi_effect
    );
}

