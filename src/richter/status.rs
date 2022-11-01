use super::*;

use globals::*;


/*
#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn richter_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    return false.into();
}

#[status_script(agent = "richter", status = FIGHTER_RICHTER_STATUS_KIND_SPECIAL_S_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn richter_special_s_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    return original!(fighter);
}
*/

pub fn install() {
    /*
    install_status_scripts!(
        richter_special_s_pre,
        richter_special_s_dash_pre
    );
    */
}
