use super::*;

#[acmd_script( agent = "richter_coffin", script = "effect_finalstart" , category = ACMD_EFFECT )]
unsafe fn coffin_finalsmash_start_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    
    /* 
    frame(weapon.lua_state_agent, 28.0);
    if is_excute(weapon) {
        EFFECT(weapon, Hash40::new("richter_final_coffin_start"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    */
}
#[acmd_script( agent = "richter_coffin", script = "effect_finalhit" , category = ACMD_EFFECT )]
unsafe fn coffin_finalsmash_hit_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    /*
    if is_excute(weapon) {
        EFFECT_OFF_KIND(weapon, Hash40::new("richter_final_coffin_start"), false, true);
        EFFECT_FOLLOW(weapon, Hash40::new("richter_final_coffin_vacuum"), Hash40::new("trans"), 0, 1, 0, 90, 0, 0, 1, true);
    }
    */
}


pub fn install() {
    install_acmd_scripts!(
        coffin_finalsmash_start_effect,
        coffin_finalsmash_hit_effect
    );
}

