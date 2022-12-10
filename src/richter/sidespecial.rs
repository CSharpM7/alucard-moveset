
use super::*;

#[acmd_script( agent = "richter", script = "game_specials1" , category = ACMD_GAME )]
unsafe fn richter_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}
#[acmd_script( agent = "richter", script = "game_specialairs1" , category = ACMD_GAME )]
unsafe fn richter_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

pub fn install() {
    install_acmd_scripts!(
        //richter_special_s_game,
        //richter_special_air_s_game
    );
}
