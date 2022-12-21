use super::*;

#[acmd_script( agent = "richter", scripts = ["effect_appeallwr","effect_entryr","effect_entryl"] , category = ACMD_EFFECT)]
unsafe fn richter_tauntlw_r_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 100.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_assist_out"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 2.0, 0,0,0,0,0,0,true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_appeallwr","game_entryr","game_entryl"] , category = ACMD_GAME)]
unsafe fn richter_tauntlw_r_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_COFFIN,false,0);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_COFFIN, true,ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_COFFIN, Hash40::new("appeal_lw_r"), false, 0.0);
    }
    for i in 2..100{
        //ModelModule::set_joint_translate(defender_boma, Hash40::new("hip"), &newPos, true,false);
        let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma), z: 0.0 };
        ArticleModule::set_pos(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_COFFIN, pos);
        wait(lua_state, 1.0);
    }
    frame(lua_state, 120.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_COFFIN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    }
}

pub fn install() {
    /*
    install_acmd_scripts!(
        richter_tauntlw_r_game,
        richter_tauntlw_r_effect
    );
    */
}
