
use super::*;

#[acmd_script( agent = "richter", script = "game_specialn" , category = ACMD_GAME , low_priority)]
unsafe fn richter_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ArticleModule::shoot(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}
#[acmd_script( agent = "richter", script = "game_specialnair" , category = ACMD_GAME , low_priority)]
unsafe fn richter_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ArticleModule::shoot(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_special_n_game,
        richter_special_air_n_game
    );
}
