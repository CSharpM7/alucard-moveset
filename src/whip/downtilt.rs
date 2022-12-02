use super::*;

#[acmd_script( agent = "richter_whip", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn whip_attack_lw3_game(fighter: &mut L2CAgentBase) {

}
#[acmd_script( agent = "richter_whip", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn whip_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot6"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        whip_attack_lw3_game,
        whip_attack_lw3_effect
    );
}

