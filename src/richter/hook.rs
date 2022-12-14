use super::*;
use skyline::{install_hook};
use {
    smash::{
        app::{lua_bind::*, FighterManager, *},
    }
};

static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
/*
#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(
fighter_manager: &mut FighterManager,
attacker_id: u32,
defender_id: u32, 
move_type: f32,
arg5: i32,
move_type_again: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let defender_boma = sv_battle_object::module_accessor(defender_id);
    let attacker_kind = sv_battle_object::kind(attacker_id);
    let attacker_entry = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let attacker_cat = utility::get_category(&mut *attacker_boma);
    let defender_cat = utility::get_category(&mut *defender_boma);
    
    println!("?!?");
    app::FighterUtil::flash_eye_info(attacker_boma);
    if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if (attacker_kind == *FIGHTER_KIND_RICHTER 
        // && WorkModule::is_flag(attacker_boma, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL) 
        ){
            println!("?");
            if (utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER) {
                opff::set_dive_target(attacker_entry, (*defender_boma).battle_object_id);

                println!("gotcha!");
                println!("{}",(*defender_boma).battle_object_id);
            }
        }
    }
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again)
}
*/
#[skyline::hook(offset=0x675a20)]
pub unsafe fn notify_log_event_collision_hit_replace(
    fighter_manager: *mut smash::app::FighterManager,
    attacker_object_id: u32,
    defender_object_id: u32, 
    move_type: f32,
    arg5: i32,
    move_type_again: bool) -> u64 {
        println!("!!");
        let attacker_boma = sv_battle_object::module_accessor(attacker_object_id);
        let defender_boma = sv_battle_object::module_accessor(defender_object_id);
        let attacker_fighter_kind = sv_battle_object::kind(attacker_object_id);
        let defender_fighter_kind = sv_battle_object::kind(defender_object_id);
        
        original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
    }

use skyline::hooks::{getRegionAddress, Region};

pub fn install() {    
    println!("[smashline_alucard::main] Loading hook");
    skyline::install_hook!(notify_log_event_collision_hit_replace);
}