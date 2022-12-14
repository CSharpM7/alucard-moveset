
use super::*;

const MOTION_RATE: f32 = 0.875;
const FRAME_ATTACK: f32 = 42.0;
pub const FRAME_END: f32 = 75.0;
const ANGLE_NEGATIVE_FACTOR: f32 = 100.0;
const ANGLE_MIN: f32 = 35.0;
const ANGLE_MAX: f32 = 60.0;

const HURTBOX_BAT: i32 = 2;

#[acmd_script( agent = "richter", scripts = ["game_specialhi","game_specialairhi"] , category = ACMD_GAME )]
unsafe fn richter_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN,false,0);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, false,ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);

        GetVar::set_int(boma, &mut vars::SPECIAL_HI_TYPE, 
            (if vars::meta_is_active(boma) {vars::SPECIAL_S_DARK} else {vars::SPECIAL_S_NORMAL})
        );
    }
    if (fighter.is_situation(*SITUATION_KIND_AIR))
    {
        FT_MOTION_RATE(fighter,0.5);
        frame(lua_state, 6.0);
        FT_MOTION_RATE(fighter,1.0);
    }    
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter,MOTION_RATE);
    if is_excute(fighter) {

        ATTACK(fighter, 0, 0, Hash40::new("hip"), 1.1, 367, 45, 0, 30, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_no_finish_camera(boma, 0, true, true);
    }

    frame(lua_state, FRAME_ATTACK-1.0);
    FT_MOTION_RATE(fighter,1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES_NEAR);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, true,ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, 
            if GetVar::get_int(boma, &mut vars::SPECIAL_HI_TYPE) == vars::SPECIAL_S_DARK {Hash40::new("fly_dark")} else {Hash40::new("fly")},
        true, 0.0);

    }
    wait(lua_state, 1.0);
    let mut angle = 361.0;
    if is_excute(fighter) {
		let entry = get_entry(fighter);
        let degree = GetVar::get_float(boma, &mut vars::BAT_DEGREE);
        
        angle = degree.clamp(ANGLE_MIN,ANGLE_MAX);
        if (degree<0.0) {angle+= degree/ANGLE_NEGATIVE_FACTOR;}
        PostureModule:: set_rot(
            fighter.module_accessor,
            &Vector3f{x: -degree, y: 0.0, z: 0.0},
            0
        ); 
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_RICHTER_GENERATE_ARTICLE_COFFIN, 
            if GetVar::get_int(boma, &mut vars::SPECIAL_HI_TYPE) == vars::SPECIAL_S_DARK {1.75} else {1.0});
        
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, angle as u64, 70, 0, 75, 6.0, 0.0, 0.0, 0.0, None,None,None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_BODY);

        for i in 0..10{
            if i != HURTBOX_BAT{
                HIT_NO(fighter, i as u64, *HIT_STATUS_OFF);
            }
        }

        if GetVar::get_int(boma, &mut vars::SPECIAL_HI_TYPE) == vars::SPECIAL_S_DARK
        {
            damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS,0);
            //GetVar::add_int(boma,&mut vars::META_FRAME,-60);
            vars::meta_loss(boma, -60);
        }
        else
        {
            damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6.0);
        }

    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, angle as u64, 70, 0, 75, 5.0, 0.0, 0.0, 0.0, None,None,None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_BODY);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        if GetVar::get_int(boma, &mut vars::SPECIAL_HI_TYPE) == vars::SPECIAL_S_NORMAL
        {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        }
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, angle as u64, 70, 0, 75, 3.5, 0.0, 0.0, 2.0, None,None,None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_BODY);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
    
}

#[acmd_script( agent = "richter", scripts = ["effect_specialhi","effect_specialairhi"] , category = ACMD_EFFECT )]
unsafe fn richter_special_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_final_coffin_start"), Hash40::new("hip"), 0,0,0,0,0,0, 0.625, true);
        LAST_EFFECT_SET_RATE(fighter,4.5);
        LAST_EFFECT_SET_COLOR(fighter,1,0,4);
    }
    frame(lua_state, FRAME_ATTACK-2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_damage_curse"), Hash40::new("top"), -5, 7.5, 2, 0, 0, 0, 1.5, 0,0,0,0,0,0,true);
        LAST_EFFECT_SET_COLOR(fighter,1,0,1);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.5, true);
    }
    frame(lua_state, FRAME_ATTACK);
    if GetVar::get_int(boma, &mut vars::SPECIAL_HI_TYPE) == vars::SPECIAL_S_DARK
    {
        if is_excute(fighter) {
            FLASH(fighter, 1, 1, 0.4, 0.3);
        }
    }
    wait(lua_state, 6.0);
    if GetVar::get_int(boma, &mut vars::SPECIAL_HI_TYPE) == vars::SPECIAL_S_DARK
    {
        if is_excute(fighter) {
            FLASH(fighter, 1, 0.7, 0.7, 0.3);
        }
    }
    wait(lua_state, 5.0);
    if GetVar::get_int(boma, &mut vars::SPECIAL_HI_TYPE) == vars::SPECIAL_S_DARK
    {
        if is_excute(fighter) {
            FLASH(fighter, 1, 0.7, 0.7, 0.3);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, FRAME_END-1.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

#[acmd_script( agent = "richter", scripts = ["sound_specialhi","sound_specialairhi"] , category = ACMD_SOUND )]
unsafe fn richter_special_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_jump03"));
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_VC(fighter, Hash40::new("vc_richter_special_h01"),0.375);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_appear01"));
    }
    frame(lua_state, FRAME_ATTACK);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_richter_special_h01"));
    }
}


#[acmd_script( agent = "richter", script = "game_attacklw32landing" , category = ACMD_GAME )]
unsafe fn richter_special_hi_landing_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter,1.5);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 65, 80, 0, 75, 7.5, 0.0, 5.0, 0.0, None,None,None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_BODY);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_special_hi_game,
        richter_special_hi_landing_game,

        richter_special_hi_effect,
        richter_special_hi_sound
    );
}
