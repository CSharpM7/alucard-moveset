
use super::*;
pub static mut SPAWN_TYPE:[i32;8] = [-1; 8];
pub const SPAWN_TYPE_HELLFIRE: i32 = 1;
pub const SPAWN_TYPE_INFERNO: i32 = 2;
pub const CHECK_FRAME: f32 = 15.0;
pub const SPAWN_FRAME: f32 = 50.0;
const SPAWN_OFFSET: f32 = 5.0;

#[acmd_script( agent = "richter", scripts = ["game_specialn","game_specialairn"] , category = ACMD_GAME )]
unsafe fn richter_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let entry = get_entry(fighter);


    frame(lua_state, SPAWN_FRAME+SPAWN_OFFSET);
    let mut projectile = *FIGHTER_RICHTER_GENERATE_ARTICLE_CROSS;
    if (GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN)==SPAWN_TYPE_INFERNO)
    {
        projectile = *FIGHTER_RICHTER_GENERATE_ARTICLE_AXE;
        GetVar::add_int(boma,&mut vars::META_FRAME,-120);
    }
    let canspawn = GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN) >= SPAWN_TYPE_HELLFIRE;
    
    if is_excute(fighter) && canspawn  {
        ArticleModule::generate_article(boma, projectile,false,0);
        ArticleModule::shoot(boma, projectile, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

#[acmd_script( agent = "richter", scripts = ["effect_specialn","effect_specialairn"] , category = ACMD_EFFECT )]
unsafe fn richter_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let entry = get_entry(fighter);

    frame(fighter.lua_state_agent, CHECK_FRAME+2.0);
    if is_excute(fighter) {
        if vars::meta_is_active(boma)
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL){
            EFFECT_FOLLOW(fighter, Hash40::new("sys_sscope_hold"), Hash40::new("top"), 5, 9.5, -5, 0, 0, 0, 0.6, true);
        }
    }
    frame(fighter.lua_state_agent, SPAWN_FRAME);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_sscope_hold"), false,false);
    }
    frame(fighter.lua_state_agent, SPAWN_FRAME);
    let canspawn = GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN) >= SPAWN_TYPE_HELLFIRE;
    if is_excute(fighter) {
        if canspawn {
            //sys_smash_flash_s
            EFFECT_FOLLOW(fighter, Hash40::new("sys_firstplace"), Hash40::new("top"), 5, 7.5, -5, 0, 0, 0, 0.4, true);
        }
    }
    frame(fighter.lua_state_agent, SPAWN_FRAME+SPAWN_OFFSET);
    if is_excute(fighter) {
        if canspawn {
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            if GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN) == SPAWN_TYPE_INFERNO
            {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_curse"), Hash40::new("hip"), 0,0,0,0,0,0, 1.25, false);
            }
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_blank"), Hash40::new("top"), 0, 8, 8, 0, 0, 0, 1, true);
        }
    }
}
#[acmd_script( agent = "richter", scripts = ["sound_specialn","sound_specialairn"] , category = ACMD_SOUND )]
unsafe fn richter_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let entry = get_entry(fighter);

    frame(fighter.lua_state_agent, CHECK_FRAME+1.0);
    if is_excute(fighter) {
        if vars::meta_is_active(boma)
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL){
        PLAY_SE(fighter, Hash40::new("se_richter_special_s01"));
        }
    }

    frame(fighter.lua_state_agent, SPAWN_FRAME);
    let canspawn = GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN) >= SPAWN_TYPE_HELLFIRE;
    if is_excute(fighter) && canspawn {
        let sound = if GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN) == SPAWN_TYPE_INFERNO {Hash40::new("vc_richter_ottotto")} else {Hash40::new("vc_richter_special_n01")};
        PLAY_VC(fighter, sound,1.0);
    }
    frame(fighter.lua_state_agent, SPAWN_FRAME+SPAWN_OFFSET);
    if is_excute(fighter) && canspawn {
        STOP_SE(fighter, Hash40::new("se_richter_special_s01"));
        PLAY_SE(fighter, Hash40::new("se_richter_special_n01"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_special_n_game,
        richter_special_n_effect,
        richter_special_n_sound
    );
}
