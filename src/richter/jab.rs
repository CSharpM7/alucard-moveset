use super::*;
const LENGTH: f32 = 15.0;

#[acmd_script( agent = "richter", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();frame(lua_state, 5.0);
    frame(lua_state, 0.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 2.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 10, 0, 10, 3.2, 0.0, 11.0, 5.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 180, 10, 0, 30, 3.2, 0.0, 11.0, 10.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, /*ID*/ 1, /*Frames*/ 1.0, /*Unk*/ false);

        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

#[acmd_script( agent = "richter", script = "sound_attack11" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_11_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

}
#[acmd_script( agent = "richter", script = "effect_attack11" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_11_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 14.5, 4, 0, -10, 0, 0.35, true, *EF_FLIP_YZ, 0.6);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), -1, 14.5, 12, 0, 0, 0, 0.5, true, *EF_FLIP_YZ, 0.5);
    }
}

#[acmd_script( agent = "richter", script = "game_attack12" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 35, 3.0, 0.0, 9.5, 3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 35, 3.0, 0.0, 9.5, 7.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 3.5, 0.0, 9.5, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
}
#[acmd_script( agent = "richter", script = "sound_attack12" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_12_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

}
#[acmd_script( agent = "richter", script = "effect_attack12" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_12_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("top"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if is_excute(fighter) {
        if(sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR)==0.0){;
            EFFECT_FOLLOW_arg11(fighter,Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 9.5, 2.5, -15, -70, 35, 0.9, true, 1);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
        else{
            EFFECT_FOLLOW_arg11(fighter,Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 9, 3, -14, -67, 33, 0.9, true, 1);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    }

}


#[acmd_script( agent = "richter", script = "game_attack13" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_13_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();frame(lua_state, 5.0);

    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 30, 50, 0, 75, 4.5, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(8.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}
#[acmd_script( agent = "richter", script = "sound_attack13" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_13_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_attackair_f01"));
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_richter_rnd_attack"));
    }
}

#[acmd_script( agent = "richter", script = "effect_attack13" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_13_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), true, true);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW_WORK(fighter, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        //LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.4);
        //AFTER_IMAGE4_ON_arg29(Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 3, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("top"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        //AFTER_IMAGE_OFF(fighter, 4);
        //EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

#[acmd_script( agent = "richter", script = "expression_attack13" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn richter_attack_13_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if is_excute(fighter) {
        //methodlib::L2CValue::as_hash()const(Hash40::new("haver"), *ATTACK_DIRECTION_Z, *ATTACK_DIRECTION_Y, *ATTACK_DIRECTION_X);
        //AttackModule::set_attack_reference_joint_id(boma);
        slope!(fighter,*MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor,Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashs"), 0);
        ControlModule::set_rumble(fighter.module_accessor,Hash40::new("rbkind_nohits"), 0, false, 0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        slope!(fighter,*MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}



#[acmd_script( agent = "richter", script = "game_attack100" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_100_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=1)
		for(1000000 Iterations){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=10, FKB=0, BKB=10, Size=7.3, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=15.0, Hitlag=0.5, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=8)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
            }
            wait(Frames=1)
			for(5 Iterations){
				if(is_excute){
					ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=10, FKB=0, BKB=10, Size=7.3, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=15.0, Hitlag=0.5, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
					AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
					ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=8)
				}
				wait(Frames=1)
				if(is_excute){
					AttackModule::clear_all()
					WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
				}
				wait(Frames=2)
			}
            /*
			if(is_excute){
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=10, FKB=0, BKB=15, Size=6.0, X=0.0, Y=8.5, Z=8.0, X2=0.0, Y2=8.5, Z2=13.0, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
				AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
				ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=8)
			}
			wait(Frames=1)
			if(is_excute){
				AttackModule::clear_all()
				WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
			}
            */
			wait_loop_clear(0)
			//wait(Frames=2)
		}
	});
}
#[acmd_script( agent = "richter", script = "sound_attack100" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_100_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
}

#[acmd_script( agent = "richter", script = "effect_attack100" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_100_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

}



#[acmd_script( agent = "richter", script = "game_attack100end" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_100end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();frame(lua_state, 5.0);

    wait(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 54, 162, 0, 60, 7.5, 0.0, 8.0, 16.0, Some(0.0), Some(8.0), Some(20.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_11_game,
        richter_attack_11_sound,
        richter_attack_11_effect,

        richter_attack_12_game,
        richter_attack_12_sound,
        richter_attack_12_effect,

        richter_attack_13_game,
        richter_attack_13_sound,
        richter_attack_13_effect,
        richter_attack_13_expression,

        richter_attack_100_game,
        //richter_attack_100_sound,
        //richter_attack_100_effect,

        richter_attack_100end_game
    );
}
