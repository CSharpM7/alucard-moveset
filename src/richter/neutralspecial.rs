
use super::*;
static mut SPAWN_TYPE:[i32;8] = [-1; 8];
const SPAWN_TYPE_HELLFIRE: i32 = 0;
const SPAWN_TYPE_INFERNO: i32 = 1;

#[acmd_script( agent = "richter", scripts = ["game_specialn","game_specialairn"] , category = ACMD_GAME )]
unsafe fn richter_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(lua_state, 18.0);
    let projectile = if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
    {*FIGHTER_SIMON_GENERATE_ARTICLE_AXE} else {*FIGHTER_SIMON_GENERATE_ARTICLE_CROSS};
    let canspawn = projectile == *FIGHTER_SIMON_GENERATE_ARTICLE_AXE
    || !app::lua_bind::WorkModule::is_flag(boma, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_CROSS);
    
    if is_excute(fighter) {
        if (canspawn){
            SPAWN_TYPE[entry] = if (projectile == *FIGHTER_SIMON_GENERATE_ARTICLE_AXE) {SPAWN_TYPE_INFERNO} else {SPAWN_TYPE_HELLFIRE};
            ArticleModule::generate_article(boma, projectile,false,0);
        }
        else{
            SPAWN_TYPE[entry] = -1;
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
    let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        let canspawn = SPAWN_TYPE[entry] >= 0;
        if canspawn {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, 3, 0, 0, 0, 1, true);
            
        }
        else 
        {
            EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_blank"), Hash40::new("top"), 0, 8, 8, 0, 0, 0, 1, true);
        }

    }
}
#[acmd_script( agent = "richter", scripts = ["sound_specialn","sound_specialairn"] , category = ACMD_SOUND )]
unsafe fn richter_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        if (SPAWN_TYPE[entry]>=0)
        {
            PLAY_SE(fighter, Hash40::new("se_richter_special_n02"));
        }
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        let sound = if SPAWN_TYPE[entry] == SPAWN_TYPE_INFERNO  {Hash40::new("vc_richter_ottotto")} else {Hash40::new("vc_richter_special_n01")};

        if (SPAWN_TYPE[entry]>=0)
        {
            PLAY_VC(fighter, sound,1);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_special_n_game,
        richter_special_n_effect,
        richter_special_n_sound
    );
}
