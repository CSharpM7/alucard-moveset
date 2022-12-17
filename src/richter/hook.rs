use super::*;
use skyline::{install_hook};
use {
    smash::{
        app::{lua_bind::*, FighterManager, *},
    }
};

use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;

//this is the flag you'll be using to recognise when the search box hits properly, you can call it whatever (i think)
const FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_SEARCH_HIT : i32 = 0x200000eb; //I'm not sure if you need this exact i32 or not, but I keep it just to make sure it works right
//this is to define the hook
static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20; //yes you need this usize

#[acmd_script( agent = "zelda", script = "game_speciallw", category = ACMD_GAME )] //this isn't character-specific or anything like that
unsafe fn zelda_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
    frame(Frame=4)
    if(is_excute){
        //this turns the flag we defined on
        WorkModule::on_flag(FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
        //this is the searchbox itself, use "collision_attr_none" to make it look seamless, 
        //but for testing I'd set it to like, literally anything else so you know when the searchbox does and doesn't hit
        //also, the hitbox I used is rather small and just covers in front of zelda, so you can change the size if you want
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=0, FKB=0, BKB=0, Size=2.8, X=0.0, Y=4.5, Z=1.0, X2=0.0, Y2=4.0, Z2=5.5, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_none"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
}
frame(Frame=6)
if(is_excute){
	//this turns the flag we defined back off
    WorkModule::off_flag(FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
    AttackModule::clear_all();
}
    });
}

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id); //this declares the boma of the attacker
    let defender_boma = sv_battle_object::module_accessor(defender_id); //and this is the defender. easily named and easy to keep straight
    let attacker_kind = sv_battle_object::kind(attacker_id); //character of the attacker
	let defender_kind = sv_battle_object::kind(defender_id); //you'll never guess what this one is
    if (defender_kind != *FIGHTER_KIND_KOOPAG) {
        //this next line checks whether the attacker is zelda, and if the flag we defined is on
        //if (attacker_kind == *FIGHTER_KIND_ZELDA && WorkModule::is_flag(attacker_boma, FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_SEARCH_HIT)) {
        //if (attacker_kind == *FIGHTER_KIND_ZELDA && WorkModule::is_flag(attacker_boma, FIGHTER_ZELDA_INSTANCE_WORK_ID_FLAG_SEARCH_HIT)) {
        if (attacker_kind == *FIGHTER_KIND_RICHTER && (*attacker_boma).is_status(*FIGHTER_STATUS_KIND_SPECIAL_S)) {
            //this makes sure it's a fighter that's being hit. you could experiment a bit with it cause I doubt it covers assist trophies but prolly best to leave this alone
            if (utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER) {
                //in here, put the action you want to happen, whether it's a hitbox or an effect or have_item or whatever
                //just make sure to use attacker_boma if it freaks out, here's my example (gives zelda a banana)
                //ItemModule::have_item(attacker_boma, smash::app::ItemKind(*ITEM_KIND_BANANA), 0, 0, false, false);
                let attacker_entry = WorkModule::get_int(&mut *attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

                if opff::get_dive_target(attacker_entry) == 0{
                    let defender_entry = WorkModule::get_int(&mut *defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    opff::set_dive_target(attacker_entry, defender_entry);

                    //CatchModule::set_catch(&mut *defender_boma, 0);

                    if ControlModule::check_button_on(&mut *attacker_boma, *CONTROL_PAD_BUTTON_ATTACK) {
                        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_DAMAGE_FALL, false);
                    }
                    else if ControlModule::check_button_on(&mut *attacker_boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_DAMAGE, false);
                    }
                    else if ControlModule::check_button_on(&mut *attacker_boma, *CONTROL_PAD_BUTTON_JUMP) {
                        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_CAPTURE_YOSHI, false);
                    }
                    else if ControlModule::check_button_on(&mut *attacker_boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_SWALLOWED, false);
                    }
                    else if ControlModule::check_button_on(&mut *attacker_boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD, false);
                    }
                    else if ControlModule::check_button_on(&mut *attacker_boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, false);
                    }
                    else if ControlModule::check_button_on(&mut *attacker_boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_DAMAGE_FLY, false);
                    }
                    else
                    {
                        (*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE, false);
                    }
                    //(*defender_boma).change_status_req(*FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE, false);
                    (*attacker_boma).change_status_req(*FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, false);
                    WorkModule::on_flag(defender_boma, *FIGHTER_STATUS_THROWN_WORK_FLAG_DISABLE_PASSIVE) ;
                    //WorkModule::set_customize_no(module_accessor, arg2, arg3)
                    //CaptureModule::capture(&mut *defender_boma, attacker_entry as c_uint,0,false,0);
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

pub fn install() {
	unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
    /* 
    install_acmd_scripts!(
	//you can rename this tho
		zelda_downb,
    );
    */
	skyline::install_hook!(notify_log_event_collision_hit_replace);
}