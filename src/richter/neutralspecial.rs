
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
        //GetVar::add_int(boma,&mut vars::META_FRAME,-120);
        vars::meta_loss(boma, -120);
    }
    let canspawn = GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN) >= SPAWN_TYPE_HELLFIRE;
    
    if is_excute(fighter) && canspawn  {
        ArticleModule::generate_article(boma, projectile,false,0);
        ArticleModule::shoot(boma, projectile, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        let article_boma = get_article_boma(boma, projectile);

        let lr = PostureModule::lr(boma);
        let xOffset = if GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN) == SPAWN_TYPE_HELLFIRE {14.0} else {8.0};
        let yOffset = if GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN) == SPAWN_TYPE_HELLFIRE {10.0} else {2.0};
        let mut pos = Vector3f::zero();
        let offset = ModelModule::joint_global_offset_from_top(boma, Hash40{hash: hash40("trans")}, &mut pos);        
        let newPos = Vector3f{x: PostureModule::pos_x(boma) + pos.x - (xOffset*lr), y: PostureModule::pos_y(boma) + pos.y + yOffset, z: PostureModule::pos_z(boma) + pos.z};
        PostureModule::set_pos(article_boma, &newPos);
        //ModelModule::set_joint_translate(article_boma, Hash40::new("root"), &newPos, true,false);
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
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
        && !ArticleModule::is_exist(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_AXE)
        {
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
            EFFECT_FOLLOW(fighter, Hash40::new("sys_firstplace"), Hash40::new("top"), 0, 7.5, -5, 0, 0, 0, 0.4, true);
        }
    }
    frame(fighter.lua_state_agent, SPAWN_FRAME+SPAWN_OFFSET);
    if is_excute(fighter) {
        if canspawn {
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_blank"), Hash40::new("top"), 5, 8, -8, 0, 0, 0, 1, true);
        }
    }
}
#[acmd_script( agent = "richter", scripts = ["sound_specialn","sound_specialairn"] , category = ACMD_SOUND )]
unsafe fn richter_special_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let entry = get_entry(fighter);

    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_dash_turn"));
    }

    frame(fighter.lua_state_agent, CHECK_FRAME+2.0);
    if is_excute(fighter) {
        if vars::meta_is_active(boma)
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
        && !ArticleModule::is_exist(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_AXE)
        {
            PLAY_SE(fighter, Hash40::new("se_richter_special_s01"));
        }
    }

    frame(fighter.lua_state_agent, SPAWN_FRAME);
    let canspawn = GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN) >= SPAWN_TYPE_HELLFIRE;
    println!("{}",GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN));
    if is_excute(fighter) && canspawn {
        STOP_SE(fighter, Hash40::new("se_richter_special_s01"));
        let sound = if GetVar::get_int(boma,&mut vars::SPECIAL_N_SPAWN) == SPAWN_TYPE_INFERNO {Hash40::new("vc_richter_ottotto")} else {Hash40::new("vc_richter_special_n01")};
        PLAY_VC(fighter, sound,1.0);
    }
    frame(fighter.lua_state_agent, SPAWN_FRAME+SPAWN_OFFSET);
    if is_excute(fighter){
        if canspawn {
            PLAY_SE(fighter, Hash40::new("se_richter_special_n01"));
        }
        else
        {
            PLAY_SE(fighter, Hash40::new("se_common_step_cloud"));
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
