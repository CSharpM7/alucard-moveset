use super::*;
utils::import_noreturn!(common::opff::fighter_common_opff);


static mut BEAKBOMB_ACTIVE:[bool;8] = [false; 8];
const HUD_DISPLAY_TIME_MAX: i32 = 90;

unsafe fn on_rebirth(fighter: &mut L2CFighterCommon,entry: usize)
{
}

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
fn richter_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
		let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
        if (fighter.is_status(*FIGHTER_STATUS_KIND_REBIRTH))
        {
            on_rebirth(fighter,entry);
        }
    }
}

#[fighter_reset]
fn richter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
		let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if fighter.kind() == *FIGHTER_KIND_RICHTER {
            on_rebirth(fighter,entry);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        richter_update
    );
    install_agent_resets!(richter_reset);
}