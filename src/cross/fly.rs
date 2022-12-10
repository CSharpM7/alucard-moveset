use super::*;

#[acmd_script( agent = "richter_cross", script = "game_fly" , category = ACMD_GAME )]
unsafe fn cross_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 6.0, 65, 60, 0, 75, 1.2, 0.0, 3.7, 0.0, Some(0.0), Some(-3.7), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 6.0, 65, 60, 0, 75, 1.2, 0.0, 0.0, 3.7, Some(0.0), Some(0.0), Some(-3.7), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "richter_cross", script = "game_turn" , category = ACMD_GAME )]
unsafe fn cross_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 5.0, 75, 95, 0, 55, 1.2, 0.0, 3.7, 0.0, Some(0.0), Some(-3.7), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 5.0, 75, 95, 0, 55, 1.2, 0.0, 0.0, 3.7, Some(0.0), Some(0.0), Some(-3.7), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);

        let motion_vec = Vector3f{x: 0.0, y: 10.0, z: 0.0};
        KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
    }
}

pub fn install() {
    install_acmd_scripts!(
        cross_fly_game,
        cross_turn_game,
    );
}

