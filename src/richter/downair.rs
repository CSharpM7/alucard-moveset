use super::*;

#[acmd_script( agent = "richter", script = "game_attackairlw" , category = ACMD_GAME )]
unsafe fn richter_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 0, 0.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
    WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1);
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 1.6, -3.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        JostleModule::set_status(boma, false);
    }
    let y = 0.0;
    let z = 2.0;
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        // Ground-only
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 72, 55, 0, 88, 6.5, 0.0, y,z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // Air-only
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 280, 85, 0, 35, 6.0, 0.0, y,z, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}


pub fn install() {
    install_acmd_scripts!(
        richter_attack_air_lw_game
    );
}
