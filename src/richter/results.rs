use super::*;

#[acmd_script( agent = "richter", scripts = ["game_entryr","game_entryl"] , category = ACMD_GAME)]
unsafe fn richter_entry_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let mut coffin_boma = fighter.module_accessor;
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN,false,0);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, true,ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, Hash40::new("appeal_lw_r"), false, 0.0);
    }
    for i in 1..10{
        if is_excute(fighter) {
            snap_article(fighter.module_accessor,*FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN,Hash40::new("trans"),Hash40::new("root"));
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 120.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    }
}


#[acmd_script( agent = "richter", scripts = ["sound_win3"] , category = ACMD_SOUND )]
unsafe fn richter_win3_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_win03_01"));
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter,Hash40::new("vc_richter_win03"));
    }
    frame(lua_state, 115.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_win03_02"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_entry_game,
        richter_win3_sound
    );
}
