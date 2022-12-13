use super::*;

#[acmd_script( agent = "richter", script = "effect_appeallwl" , category = ACMD_EFFECT)]
unsafe fn richter_tauntlw_l_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_dead"), Hash40::new("top"), 10,50,0,0,0,0, 0.75, true);
        LAST_PARTICLE_SET_COLOR(fighter, 1.0, 1.0,0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
    }
}
#[acmd_script( agent = "richter", script = "effect_appeallwr" , category = ACMD_EFFECT)]
unsafe fn richter_tauntlw_r_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_assist_out"), Hash40::new("top"), 0,-2,0,0,0,0, 2.0, true);
        LAST_PARTICLE_SET_COLOR(fighter, 1.0, 1.0,0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_assist_out"), Hash40::new("top"), 0,-2,0,0,0,0, 2.0, true);
        LAST_PARTICLE_SET_COLOR(fighter, 1.0, 1.0,0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_tauntlw_l_effect,
        richter_tauntlw_r_effect
    );
}
