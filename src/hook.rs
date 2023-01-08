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
static mut FIGHTER_HANDLE_DAMAGE_OFFSET : usize = 0x6310a0; //yes you need this usize

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
            //Side Special not-Command Grab
            if (*attacker_boma).is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) 
            && GetVar::get_int(attacker_boma,&mut vars::DIVE_TARGET) == 0 
            && GetVar::get_int(attacker_boma, &mut vars::SPECIAL_S_TYPE) > 0 {

                GetVar::set_int(attacker_boma, &mut vars::DIVE_TARGET,defender_entry);
                KineticModule::clear_speed_all(attacker_boma);
                KineticModule::change_kinetic(defender_boma, *FIGHTER_KINETIC_TYPE_CAPTURE);
                KineticModule::change_kinetic(attacker_boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                HitModule::set_status_all(defender_boma,HitStatus(*HIT_STATUS_XLU),0);

                if GetVar::get_int(attacker_boma, &mut vars::SPECIAL_S_TYPE) == vars::SPECIAL_S_NORMAL
                {

                    let mut defenderState = *FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE;
                    /*
                    *FIGHTER_STATUS_KIND_SWIM_DROWN_OUT; //Has airbubble unfortunately
                    *FIGHTER_STATUS_KIND_PSYCHOBREAK; //oddly works but is vulnerable
                    *FIGHTER_STATUS_KIND_SIMON_FINAL_TARGET_START; //Same as Blackhole
                    *FIGHTER_STATUS_KIND_GAOGAEN_FINAL_TARGET_START; //Turns foes around, anim ends too soon
                    */
                    if ControlModule::check_button_on(attacker_boma , *CONTROL_PAD_BUTTON_ATTACK) {
                        defenderState = *FIGHTER_STATUS_KIND_PSYCHOBREAK; //oddly works but is vulnerable
                    }
                    else if ControlModule::check_button_on(attacker_boma , *CONTROL_PAD_BUTTON_SPECIAL) {
                        defenderState = *FIGHTER_STATUS_KIND_SHULK_FINAL_START;
                    }
                    else if ControlModule::check_button_on(attacker_boma , *CONTROL_PAD_BUTTON_JUMP) {
                        defenderState = *FIGHTER_STATUS_KIND_JACK_FINAL_TARGET_START; //He's just standing there
                    }
                    else if ControlModule::check_button_on(attacker_boma , *CONTROL_PAD_BUTTON_GUARD) {
                        defenderState = *FIGHTER_STATUS_KIND_SAVING_DAMAGE;
                    }
                    else if ControlModule::check_button_on(attacker_boma , *CONTROL_PAD_BUTTON_APPEAL_HI) {
                        defenderState = *FIGHTER_STATUS_KIND_SWIM_DROWN_OUT; //requires posture change
                    }
                    else if ControlModule::check_button_on(attacker_boma , *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                        defenderState = *FIGHTER_STATUS_KIND_SAVING_DAMAGE_AIR;
                    }
                    else if ControlModule::check_button_on(attacker_boma , *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                        defenderState = *FIGHTER_STATUS_KIND_GAOGAEN_FINAL_TARGET_START; //Turns foes around, anim ends too soon
                    }
                    else if ControlModule::check_button_on(attacker_boma , *CONTROL_PAD_BUTTON_APPEAL_LW) {
                        defenderState = *FIGHTER_STATUS_KIND_ROULETTE_FURAFURA; //Hmmm
                    }
                    defenderState = *FIGHTER_STATUS_KIND_SWIM_DROWN_OUT; //requires posture change
                    (*defender_boma).change_status_req(defenderState, false);
                    PostureModule::set_lr(defender_boma, PostureModule::lr(attacker_boma)*-1.0);
                    HitModule::set_status_all(defender_boma,HitStatus(*HIT_STATUS_XLU),0);
                    WorkModule::on_flag(defender_boma, *FIGHTER_STATUS_THROWN_WORK_FLAG_DISABLE_PASSIVE);
                    (*attacker_boma).change_status_req(*FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, false);
                    //HitModule::set_whole(defender_boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                    //WorkModule::set_customize_no(module_accessor, arg2, arg3)
                    //CaptureModule::capture(&mut *defender_boma, attacker_entry as c_uint,0,false,0);
                }
                else if GetVar::get_int(attacker_boma, &mut vars::SPECIAL_S_TYPE) == vars::SPECIAL_S_DARK
                {
                    GetVar::set_int(attacker_boma, &mut vars::SPECIAL_S_TYPE,vars::SPECIAL_S_DARK_ACTIVE);
                    (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_DAMAGE, false);

                    let motion = if (*attacker_boma).is_situation(*SITUATION_KIND_AIR) {"attack_hi3"} else {"attack_hi3"};
                    MotionModule::change_motion(attacker_boma,Hash40::new(motion),1.0,1.0,false,0.0,false,false);
                }
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
                    if MotionModule::frame(attacker_boma) > 115.0{
                        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_DAMAGE, false);
                    }
                    else if !(*defender_boma).is_status(*FIGHTER_STATUS_KIND_PSYCHOBREAK) {
                        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_PSYCHOBREAK, false);
                    }
                }
            }
        }
    }
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again)
}

#[skyline::hook(offset = FIGHTER_HANDLE_DAMAGE_OFFSET)]
unsafe fn fighter_handle_damage_hook(defender: *mut smash::app::BattleObject, arg: *const u8) {

    let defender_boma = (*defender).module_accessor;
    // let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let damage_received = WorkModule::get_float(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
    // let hitstun = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    // println!("histun remaining: {}", hitstun);
    call_original!(defender, arg);
    let damage_received = WorkModule::get_float(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE) - damage_received;
    let attacker_ids = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID);
    // println!("attacker ids: {}", attacker_ids);
    let defender_kind = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if (defender_kind == *FIGHTER_KIND_RICHTER)
    {
        let meterloss = (damage_received*3.0).ceil();
        println!("Richter got hit for: {}", damage_received);
        vars::meta_loss(defender_boma, -meterloss as i32);
    }
    for x in 0..8 {
        if attacker_ids & (1 << x) == 0 {
            continue;
        }
        if let Some(object_id) = get_active_battle_object_id_from_entry_id(x) {
            let attacker_object = get_battle_object_from_id(object_id);
            let attacker_boma = (*attacker_object).module_accessor;
            let attacker_kind = utility::get_kind(&mut *attacker_boma);
            if attacker_kind == *FIGHTER_KIND_RICHTER {
                println!("Richter hit for: {}", damage_received);
                vars::meta_heal(attacker_boma,damage_received);
                /*
                let meterloss = (damage_received*3.0).ceil();
                vars::meta_loss(attacker_boma, -meterloss as i32); */
            }
        }
    }

}

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

#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;
    let mut clear_buffer = arg3;

    if boma.is_fighter() {
        if boma.kind() == *FIGHTER_KIND_RICHTER
        && meta_swordattack(boma)
        {
            if (GetVar::get_bool(boma, &mut vars::META_WHIFF) && !GetVar::get_bool(boma, &mut vars::META_HEALED))
            {
                println!("richter whiffed an attack");
                let mut frameloss = META_PUNISH;
                GetVar::add_int(boma,&mut vars::META_FRAME,frameloss);
                EffectModule::req_follow(boma, Hash40::new("sys_hit_curse"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 1.25, true, 0, 0, 0, 0, 0, false, false);
            }
            GetVar::set_bool(boma, &mut vars::META_WHIFF,false);
        }
        /* 
        && StatusModule::status_kind(boma) == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH
        && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
        && next_status == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP {
            next_status = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
        }*/
    }
    original!()(boma, next_status, clear_buffer)
}


fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static COLLISION_HIT_SEARCH_CODE: &[u8] = &[
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
pub fn install() {
	unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, COLLISION_HIT_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
	//skyline::install_hook!();
    skyline::install_hooks!(
        notify_log_event_collision_hit_replace,
        fighter_handle_damage_hook,
        change_version_string_hook,
        change_status_request_from_script_hook
    );
}