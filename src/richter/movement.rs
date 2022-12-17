use super::*;

#[acmd_script( agent = "richter", script = "sound_run" , category = ACMD_SOUND )]
unsafe fn richter_run_sound(fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "richter", script = "effect_run" , category = ACMD_EFFECT )]
unsafe fn richter_run_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 13.0);
}

#[acmd_script( agent = "richter", script = "sound_ottotto" , category = ACMD_SOUND )]
unsafe fn richter_ottotto_sound(fighter: &mut L2CAgentBase) {
}

pub fn install() {
    install_acmd_scripts!(
        richter_run_sound,
        richter_run_effect,

        richter_ottotto_sound
    );
}
