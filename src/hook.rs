use super::*;
use skyline::{install_hook};
use {
    smash::{
        app::{lua_bind::*, FighterManager, *}
    }
};

use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;

//this is to define the hook
static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20; //yes you need this usize

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id); //this declares the boma of the attacker
    let defender_boma = sv_battle_object::module_accessor(defender_id); //and this is the defender. easily named and easy to keep straight
    let attacker_kind = sv_battle_object::kind(attacker_id); //character of the attacker
	let defender_kind = sv_battle_object::kind(defender_id); //you'll never guess what this one is
    if (defender_kind != *FIGHTER_KIND_KOOPAG)
    && (utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER) {
        let attacker_entry = get_entry_from_boma(attacker_boma);
        let defender_entry = get_entry_from_boma(defender_boma) as i32;
        if (attacker_kind == *FIGHTER_KIND_RICHTER) {
            if (*attacker_boma).is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) 
            && GetVar::get_int(attacker_boma,&mut vars::DIVE_TARGET) == 0 {
                GetVar::set_int(attacker_boma, &mut vars::DIVE_TARGET,defender_entry);

                (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE, false);
                MotionModule::change_motion_kind(defender_boma, Hash40::new("damage_n3"));
                HitModule::set_status_all(defender_boma,HitStatus(*HIT_STATUS_XLU),0);
                (*attacker_boma).change_status_req(*FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, false);
                WorkModule::on_flag(defender_boma, *FIGHTER_STATUS_THROWN_WORK_FLAG_DISABLE_PASSIVE) ;
                //WorkModule::set_customize_no(module_accessor, arg2, arg3)
                //CaptureModule::capture(&mut *defender_boma, attacker_entry as c_uint,0,false,0);
            }
            //Soul steal can hit at most ~12 times
            //1P: 4.5(*.375)
            //3P: 13.5
            //7P: 27
            //Last hit heals a set 25%
            else if (*attacker_boma).is_status(*FIGHTER_SIMON_STATUS_KIND_FINAL_END) {
                if AttackModule::get_power(attacker_boma, 0, false, 1.0, false) >= 10.0{
                    EffectModule::req_follow(defender_boma, Hash40::new("sys_item_arrival"),Hash40::new("rot"), &Vector3f::zero(), &Vector3f::zero(), 1.25, true, 0, 0, 0, 0, 0, false, false) as u64;
                    EffectModule::set_rate_last(defender_boma,0.5);

                    if DamageModule::damage(defender_boma,0) > 100.0 {
                    && 
                        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_DEAD, true);
                    }
                }
                else{
                    DamageModule::heal(attacker_boma,-0.375,0);
                }
            }
        }
    }
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again)
}

//trust me that everything from this point on does important things that you cannot mess with
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];


extern "C" {
    fn change_version_string(arg: u64, string: *const c_char);
}

#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
    let original_str = unsafe { skyline::from_c_str(string) };
    if original_str.contains("Ver.") {

        let new_str = format!(
            "{}\nAlucard Ver. {}\0",
            original_str,
            env!("CARGO_PKG_VERSION")
        );

        call_original!(arg, skyline::c_str(&new_str))
    } else {
        call_original!(arg, string)
    }
}

pub fn install() {
	unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
	//skyline::install_hook!();
    skyline::install_hooks!(
        notify_log_event_collision_hit_replace,
        change_version_string_hook
    );
}