use super::*;

#[acmd_script( agent = "richter", scripts = ["game_entryr","game_entryl"] , category = ACMD_GAME)]
unsafe fn richter_entry_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN,false,0);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, true,ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, Hash40::new("fly"), false, 0.0);
    }
    frame(lua_state, 64.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
#[acmd_script( agent = "richter", scripts = ["effect_entryr","effect_entryl"] , category = ACMD_EFFECT)]
unsafe fn richter_entry_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 63.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_damage_curse"), Hash40::new("top"), -5, 7.5, 2, 0, 0, 0, 1.5, 0,0,0,0,0,0,true);
    }
}
#[acmd_script( agent = "richter", scripts = ["sound_entryr","sound_entryl"] , category = ACMD_SOUND)]
unsafe fn richter_entry_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_special_h01"));
    }
}

#[acmd_script( agent = "richter", script = "effect_win1" , category = ACMD_EFFECT )]
unsafe fn richter_win1_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 124.0);
}

#[acmd_script( agent = "richter", script = "sound_win1" , category = ACMD_SOUND )]
unsafe fn richter_win1_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
    PLAY_SE_NO_3D(fighter,Hash40::new("vc_richter_win01"));
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
    //PLAY_SE(fighter, Hash40::new("se_richter_win01_01"));
    }
    frame(fighter.lua_state_agent, 124.0);
    if is_excute(fighter) {
    PLAY_SE_NO_3D(fighter,Hash40::new("se_richter_win01_02"));
    }
}

#[acmd_script( agent = "richter", script = "effect_win2" , category = ACMD_EFFECT )]
unsafe fn richter_win2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
    EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), -0.0, -5.0, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
    EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), -0.0, -5.0, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
    EFFECT(fighter, Hash40::new("chrom_win_3"), Hash40::new("top"), -7, -5, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
#[acmd_script( agent = "richter", script = "sound_win2" , category = ACMD_SOUND )]
unsafe fn richter_win2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter,Hash40::new("vc_richter_win02"));
    }
    /*
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_win02_01"));
    }
    frame(lua_state, 115.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_win03_02"));
    } */
}
#[acmd_script( agent = "richter", script = "sound_win3" , category = ACMD_SOUND )]
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
        richter_entry_effect,
        richter_entry_sound,

        richter_win1_effect,
        richter_win1_sound,
        richter_win2_effect,
        richter_win2_sound,
        richter_win3_sound
    );
}
