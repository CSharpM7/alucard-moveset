
use super::*;

const MOTION_RATE: f32 = 0.875;
const FRAME_ATTACK: f32 = 39.0;
const FRAME_END: f32 = 65.0;
const HURTBOX_BAT: i32 = 2;
const ANGLE_NEGATIVE_FACTOR: f32 = 100.0;
const ANGLE_MIN: f32 = 35.0;
const ANGLE_MAX: f32 = 60.0;


#[acmd_script( agent = "richter", script = "game_specialhi" , category = ACMD_GAME )]
unsafe fn richter_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }

    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter,MOTION_RATE);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 1.1, 367, 45, 0, 30, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }

    frame(lua_state, FRAME_ATTACK-1.0);
    FT_MOTION_RATE(fighter,1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES_NEAR);
    }
    wait(lua_state, 1.0);
    let mut angle = 361.0;
    if is_excute(fighter) {
		let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let degree = opff::get_degree(entry);
        angle = degree.clamp(ANGLE_MIN,ANGLE_MAX);
        if (degree<0.0) {angle+= degree/ANGLE_NEGATIVE_FACTOR;}

        for i in 0..10{
            if i != HURTBOX_BAT{
                HIT_NO(fighter, i as u64, *HIT_STATUS_OFF);
            }
        }
        damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);

        
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, angle as u64, 70, 0, 75, 8.0, 0.0, 0.0, 0.0, None,None,None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, angle as u64, 70, 0, 75, 5.5, 0.0, 0.0, 0.0, None,None,None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_BODY);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, angle as u64, 70, 0, 75, 3.5, 0.0, 0.0, 2.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_BODY);
    }
    
    frame(lua_state, FRAME_END);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE_CONTROL);

        for i in 0..10{
            if i != HURTBOX_BAT{
                HIT_NO(fighter, i as u64, *HIT_STATUS_NORMAL);
            }
        }
        PostureModule:: set_rot(
            fighter.module_accessor,
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            0
        ); 

        AttackModule::clear_all(fighter.module_accessor);
    }
    FT_MOTION_RATE(fighter,0.01);
}
#[acmd_script( agent = "richter", script = "game_specialairhi" , category = ACMD_GAME )]
unsafe fn richter_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter,0.5);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter,1.0);

    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter,MOTION_RATE);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 1.1, 367, 45, 0, 30, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }

    frame(lua_state, FRAME_ATTACK-1.0);
    FT_MOTION_RATE(fighter,1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES_NEAR);
    }
    wait(lua_state, 1.0);
    let mut angle = 361.0;
    if is_excute(fighter) {
		let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let degree = opff::get_degree(entry);
        angle = degree.clamp(ANGLE_MIN,ANGLE_MAX);
        if (degree<0.0) {angle+= degree/ANGLE_NEGATIVE_FACTOR;}

        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES_NEAR);

        for i in 0..10{
            if i != HURTBOX_BAT{
                HIT_NO(fighter, i as u64, *HIT_STATUS_OFF);
            }
        }
        damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
        
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, angle as u64, 70, 0, 75, 8.0, 0.0, 0.0, 0.0, None,None,None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, angle as u64, 70, 0, 75, 5.5, 0.0, 0.0, 0.0, None,None,None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_BODY);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, angle as u64, 70, 0, 75, 3.5, 0.0, 0.0, 2.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_BODY);
    }

    frame(lua_state, FRAME_END);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE_CONTROL);

        for i in 0..10{
            if i != HURTBOX_BAT{
                HIT_NO(fighter, i as u64, *HIT_STATUS_NORMAL);
            }
        }
        PostureModule:: set_rot(
            fighter.module_accessor,
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            0
        ); 

        AttackModule::clear_all(fighter.module_accessor);
    }
    FT_MOTION_RATE(fighter,0.01);
}

#[acmd_script( agent = "richter", script = "effect_specialhi" , category = ACMD_EFFECT )]
unsafe fn richter_special_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

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
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 0.4, 0.3);
    }
    frame(lua_state, FRAME_ATTACK+3.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 4, 0, 0, 0, 0);
    }
}
#[acmd_script( agent = "richter", script = "effect_specialairhi" , category = ACMD_EFFECT )]
unsafe fn richter_special_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
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
        EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("hip"), 0, 0, 2, 0, 0, 0, 2.5, true);
    }
    frame(lua_state, FRAME_ATTACK);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 0.4, 0.3);
    }
    frame(lua_state, FRAME_ATTACK+3.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 4, 0, 0, 0, 0);
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
        richter_special_air_hi_game,
        richter_special_hi_landing_game,

        richter_special_hi_effect,
        richter_special_air_hi_effect
    );
}
