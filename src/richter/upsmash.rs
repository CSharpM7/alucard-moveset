use super::*;
const ORIGIN_Z: f32 = 20.5;
const END_Y: f32 = 10.0;
const FAMILIAR_LENGTH: f32 = 19.0;
const FRAME_SPAWN: f32 = 20.0;
const FRAME_ATTACK: f32 = 23.0;

#[acmd_script( agent = "richter", script = "game_attackhi4" , category = ACMD_GAME )]
unsafe fn richter_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, FRAME_SPAWN-1.0);
    FT_MOTION_RATE(fighter, 1.0);
    
    let kbg=100;
    let fkb=0;
    let z=6.5;
    let angle=15;
    let damage=5.0;
    let shield_damage=10;
    //frame(lua_state, FRAME_SPAWN);
    if is_excute(fighter) {
        
        ATTACK(fighter, 0, 0, Hash40::new("throw"), damage, 368, kbg, fkb, 0, 3.0, 0.0, 0.0, 0.0, None,None,None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        ATTACK(fighter, 1, 0, Hash40::new("throw"), damage, 368, kbg, fkb, 0, 3.0, 0.0, 0.0, -z, Some(0.0), Some(0.0),Some(-z/2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("throw"), damage, 368, kbg, fkb, 0, 3.0, 0.0, 0.0, z, Some(0.0), Some(0.0),Some(z/2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(fighter,0, 3.0);
        ATK_SET_SHIELD_SETOFF_MUL(fighter,1, 3.0);
        ATK_SET_SHIELD_SETOFF_MUL(fighter,2, 3.0);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);

        let offset = smash::phx::Vector2f { x: ORIGIN_Z, y: FAMILIAR_LENGTH};
        AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("rot"), &offset, 4, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("rot"), &offset, 4, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40::new("rot"), &offset, 4, false);

    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear(boma,1,false);
        AttackModule::clear(boma,2,false);
        ATTACK(fighter, 0, 0, Hash40::new("throw"), damage, 90, kbg, fkb/5, 0, 3.5, 0.0, 3.0, 0.0, Some(0.0), Some(FAMILIAR_LENGTH-4.0), Some(0.0), 0.05, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(fighter,0, 2.0);
    }
    frame(lua_state, FRAME_ATTACK);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 15.0, 86, 90, 0, 45, 3.5, 0.0, FAMILIAR_LENGTH/1.5, 0.0, Some(0.0), Some(FAMILIAR_LENGTH+1.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("throw"), 9.0, 86, 60, 0, 60, 3.25, 0.0, FAMILIAR_LENGTH, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackhi4" , category = ACMD_EFFECT )]
unsafe fn richter_attack_hi4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 18, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_teamhealfield"), Hash40::new("top"), 0,0,ORIGIN_Z,0,0,0, 0.9, true);
        LAST_EFFECT_SET_COLOR(fighter,1.0,0.25,1.0);
    }
    frame(lua_state, FRAME_SPAWN);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_assist_out"), Hash40::new("top"), 0,0,ORIGIN_Z,0,0,0, 0.75, true);
        LAST_EFFECT_SET_RATE(fighter,1.5);
    }
    frame(lua_state, FRAME_ATTACK);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("throw"), 0, 20, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter,1.5);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_teamhealfield"), false,false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_assist_out"), Hash40::new("top"), 0,END_Y,ORIGIN_Z,0,0,0, 0.875, true);
        LAST_EFFECT_SET_RATE(fighter,1.75);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackhi4charge" , category = ACMD_EFFECT )]
unsafe fn richter_attack_hi4charge_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 5.0);
    for f in 1..20 
    {
        if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);

        //EFFECT_FOLLOW(fighter, Hash40::new("sys_teamhealfield"), Hash40::new("top"), 0,0,ORIGIN_Z,0,0,0, 0.75, true);
        //LAST_EFFECT_SET_COLOR(fighter,1.0,0.25,1.0);
        }
        else {break;}

        wait(lua_state, 5.0);
        if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("handl"), 2.0, 0, -2.5, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        wait(lua_state, 5.0);
        }
        else {break;}
    }
}


#[acmd_script( agent = "richter", script = "sound_attackhi4" , category = ACMD_SOUND )]
unsafe fn richter_attack_hi4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_h01"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack06"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_smash_h02"));
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_appear02"));
    }
}
pub fn install() {
    install_acmd_scripts!(
        richter_attack_hi4_game,
        richter_attack_hi4_effect,
        richter_attack_hi4_sound,
        richter_attack_hi4charge_effect
    );
}

