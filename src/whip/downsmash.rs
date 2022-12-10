use super::*;

#[acmd_script( agent = "richter_whip", script = "game_attacklw4" , category = ACMD_GAME )]
unsafe fn whip_attack_lw4_game(fighter: &mut L2CAgentBase) {

}
#[acmd_script( agent = "richter_whip", script = "effect_attacklw4" , category = ACMD_EFFECT )]
unsafe fn whip_attack_lw4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_blackball_set"), Hash40::new("top"), 0, 3.0, 30.0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter,2.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_shadow"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_greenshell_trace"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter,1,0.125,1);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_purple"), Hash40::new("top"), 0.0, 3.0, 30.0, 0, 0, 0, 1, true);
    }
}
#[acmd_script( agent = "richter_whip", script = "effect_attacklw4charge" , category = ACMD_EFFECT )]
unsafe fn whip_attack_lw4_charge_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    for f in (1..61).step_by(20)
    {
        frame(lua_state, f as f32);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_blackball_set"), Hash40::new("top"), 0, 3.0, 30.0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(fighter,2.0);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        whip_attack_lw4_game,
        whip_attack_lw4_effect,
        whip_attack_lw4_charge_effect
    );
}

