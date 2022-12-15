use super::*;

#[acmd_script( agent = "richter_coffin", script = "game_appeallwr" , category = ACMD_GAME )]
unsafe fn entry_r_game(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
}
#[acmd_script( agent = "richter_coffin", script = "effect_fly" , category = ACMD_EFFECT )]
unsafe fn entry_r_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
}

#[acmd_script( agent = "richter_coffin", script = "game_turn" , category = ACMD_GAME )]
unsafe fn cross_turn_game(weapon: &mut L2CAgentBase) {
}
#[acmd_script( agent = "richter_coffin", script = "effect_turn" , category = ACMD_EFFECT )]
unsafe fn cross_turn_effect(weapon: &mut L2CAgentBase) {
}


pub fn install() {
    install_acmd_scripts!(
        entry_r_game
        //entry_r_effect,
    );
}

