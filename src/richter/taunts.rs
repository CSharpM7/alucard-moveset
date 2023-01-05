use super::*;

#[acmd_script( agent = "richter", scripts = ["effect_appeallwr"] , category = ACMD_EFFECT)]
unsafe fn richter_tauntlw_r_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 100.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_assist_out"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 2.0, 0,0,0,0,0,0,true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_appeallwr"] , category = ACMD_GAME)]
unsafe fn richter_tauntlw_r_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN,false,0);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, true,ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, Hash40::new("appeal_lw_r"), true, 0.0);
        
    }
    for i in 2..100{
        if is_excute(fighter) {
            //snap_article(fighter.module_accessor,*FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN,Hash40::new("trans"),Hash40::new("root"));
            let mut pos = Vector3f::zero();
            let offset = ModelModule::joint_global_offset_from_top(boma, Hash40{hash: hash40("trans")}, &mut pos);        
            let newPos = Vector3f{x: PostureModule::pos_x(boma) + pos.x, y: PostureModule::pos_y(boma) + pos.y + 0.0, z: PostureModule::pos_z(boma) + pos.z};
            let article_boma = get_article_boma(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN);
            PostureModule::set_pos(article_boma,  &newPos);
            //ArticleModule::set_pos(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, newPos);
            //ModelModule::set_joint_translate(article_boma, Hash40::new("root"), &newPos, true,false);
            //ArticleModule::change_status(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN,*WEAPON_SIMON_COFFIN_STATUS_KIND_NORMAL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 120.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_tauntlw_r_game,
        richter_tauntlw_r_effect
    );
}
