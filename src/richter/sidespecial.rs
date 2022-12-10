
use super::*;

#[acmd_script( agent = "richter", script = "game_specials1" , category = ACMD_GAME , low_priority)]
unsafe fn richter_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.2);
    if is_excute(fighter) {
        //ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS,false,0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
    WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(fighter, 1);
    if is_excute(fighter) {
        
        ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS,false,0);
        ArticleModule::shoot(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        
        let mut pos = *PostureModule::pos(boma);
        let mut spawn: Vector3f = Vector3f::zero();
        pos.x += 20.0*PostureModule::lr(boma);
        ArticleModule::set_pos(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, pos);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
    WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
}
#[acmd_script( agent = "richter", script = "game_specialairs1" , category = ACMD_GAME , low_priority)]
unsafe fn richter_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}

pub fn install() {
    install_acmd_scripts!(
        richter_special_s_game,
        richter_special_air_s_game
    );
}
