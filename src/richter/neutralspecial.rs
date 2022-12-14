
use super::*;

#[acmd_script( agent = "richter", scripts = ["game_specialn","game_specialairn"] , category = ACMD_GAME )]
unsafe fn richter_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    frame(lua_state, 20.0);
    let projectile = //if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
    if true
    {*FIGHTER_SIMON_GENERATE_ARTICLE_AXE} else {*FIGHTER_SIMON_GENERATE_ARTICLE_CROSS};
    let canspawn = projectile == *FIGHTER_SIMON_GENERATE_ARTICLE_AXE
    || !app::lua_bind::WorkModule::is_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_CROSS);
    
    if is_excute(fighter) {
        if (canspawn){
            ArticleModule::generate_article(boma, projectile,false,0);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) && canspawn  {
        ArticleModule::shoot(boma, projectile, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

#[acmd_script( agent = "richter", scripts = ["effect_specialn","effect_specialairn"] , category = ACMD_EFFECT )]
unsafe fn richter_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        let canspawn = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) 
        || !app::lua_bind::WorkModule::is_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_CROSS);
        let effect = if canspawn {Hash40::new("sys_smash_flash_s")} else {Hash40::new("richter_bottle_blank")};
        EFFECT(fighter, effect, Hash40::new("top"), 0, 6, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
pub fn install() {
    install_acmd_scripts!(
        richter_special_n_game,
        richter_special_n_effect
    );
}
