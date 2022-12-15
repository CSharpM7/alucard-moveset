use super::*;
mod entry;

pub fn install() {
    //entry::install();
    install_status_scripts!(
        //coffin_normal_exec
    );
}


#[status_script(agent = "richter_coffin", status = WEAPON_SIMON_COFFIN_STATUS_KIND_NORMAL, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn coffin_normal_exec(weapon: &mut L2CFighterCommon) -> L2CValue {
    if (MotionModule::is_end(weapon.module_accessor) == true 
        && weapon.is_motion(Hash40::new("appeal_lw_r")))
    {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        ArticleModule::remove_exist(owner_module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_COFFIN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    }
    return original!(weapon);
}