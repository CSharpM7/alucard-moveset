use super::*;

#[acmd_script( agent = "richter_whip", script = "effect_attackdash" , category = ACMD_EFFECT )]
unsafe fn whip_attack_dash_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.module_accessor;
    
    frame(lua_state, 9.0);
    if is_excute(weapon) {
        whip_trail(weapon);
    }
    frame(lua_state, 13.0);
    if is_excute(weapon) {
        AFTER_IMAGE_OFF(weapon, 5);
    }
}
#[acmd_script( agent = "richter_whip", script = "effect_attacks3" , category = ACMD_EFFECT )]
unsafe fn whip_attack_s3_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.module_accessor;
    
    frame(lua_state, 6.0);
    if is_excute(weapon) {
        whip_trail(weapon);
    }
    frame(lua_state, 15.0);
    if is_excute(weapon) {
        AFTER_IMAGE_OFF(weapon, 5);
    }
}
#[acmd_script( agent = "richter_whip", script = "effect_attacklw3" , category = ACMD_EFFECT )]
unsafe fn whip_attack_lw3_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.module_accessor;
    
    frame(lua_state, 6.0);
    if is_excute(weapon) {
        whip_trail(weapon);
    }
    frame(lua_state, 15.0);
    if is_excute(weapon) {
        AFTER_IMAGE_OFF(weapon, 5);
    }
}

#[acmd_script( agent = "richter_whip", scripts = ["effect_attacks4hi","effect_attacks4","effect_attacks4lw"] , category = ACMD_EFFECT )]
unsafe fn whip_attack_s4_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

    frame(lua_state, 12.0);
    if is_excute(weapon) {
        whip_trail(weapon);
    }
    frame(lua_state, 20.0);
    if is_excute(weapon) {
        AFTER_IMAGE_OFF(weapon, 5);
    }
}

#[acmd_script( agent = "richter_whip", scripts = ["effect_attackairfhi","effect_attackairf","effect_attackairflw"] , category = ACMD_EFFECT )]
unsafe fn whip_attack_air_f_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = get_owner_boma(weapon);
    
    frame(lua_state, 8.0);
    if is_excute(weapon) {
        whip_trail(weapon);
    }
    frame(lua_state, 18.0);
    if is_excute(weapon) {
        AFTER_IMAGE_OFF(weapon, 5);
    }
}
#[acmd_script( agent = "richter_whip", script = "effect_attackairb" , category = ACMD_EFFECT )]
unsafe fn whip_attack_air_b_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.module_accessor;
    
    frame(lua_state, 6.0);
    if is_excute(weapon) {
        whip_trail(weapon);
    }
    frame(lua_state, 10.0);
    if is_excute(weapon) {
        AFTER_IMAGE_OFF(weapon, 5);
    }
}
pub fn install() {
    install_acmd_scripts!(
        whip_attack_s3_effect,
        whip_attack_lw3_effect,
        whip_attack_s4_effect,
        whip_attack_dash_effect,
        whip_attack_air_f_effect,
        whip_attack_air_b_effect
    );
}

