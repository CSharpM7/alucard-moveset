use super::*;

#[acmd_script( agent = "richter_whip", scripts = ["effect_attackairfhi","effect_attackairf","effect_attackairflw"] , category = ACMD_EFFECT )]
unsafe fn whip_attack_air_f_effect(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    //let boma = weapon.module_accessor;
    
    frame(lua_state, 6.0);
    if is_excute(weapon) {
        
        println!("{}",vars::meta_is_active(boma));
        let color = if (vars::meta_is_active(boma)) {Vector3f{x: 0.75,y:0.75,z:1.0}} else {Vector3f{x: 1.0,y:0.75,z:0.75}};
        let effect = if (vars::meta_is_active(boma)) {Hash40::new("tex_item_killsword2")} else {Hash40::new("tex_item_killsword2")};

        AFTER_IMAGE4_ON_arg29(weapon,effect, effect, 5, Hash40::new("whip"), 0.0, 0.0, -0.6, Hash40::new("whip"), 0.0, vars::LENGTH+2.0, 0.6, true, Hash40::new("null"), Hash40::new("whip"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);

        LAST_EFFECT_SET_COLOR(weapon, color.x,color.y,color.z);
    }
    frame(lua_state, 18.0);
    if is_excute(weapon) {
        AFTER_IMAGE_OFF(weapon, 5);
    }
}
pub fn install() {
    install_acmd_scripts!(
        whip_attack_air_f_effect
    );
}

