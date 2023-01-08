use super::*;

pub const LENGTH: f32 = 15.0;
pub const WIDTH: f32 = 3.0;

pub static mut META_HEALED:[bool;8] = [false; 8];
pub static mut META_WHIFF:[bool;8] = [false; 8];
pub static mut META_FRAME:[i32;8] = [0; 8];
pub static mut META_EFFECT:[i32;8] = [-1; 8];
pub static mut META_ATTEMPTS:[i32;8] = [-1; 8];
pub const META_MAX: i32 = 1500;
pub const META_PUNISH: i32 = 60;

pub static mut SPECIAL_N_SPAWN:[i32;8] = [0; 8];
pub static mut SPECIAL_S_AERIAL:[bool;8] = [true; 8];
pub static mut SPECIAL_S_TYPE:[i32;8] = [0; 8];
pub static mut SPECIAL_HI_TYPE:[i32;8] = [0; 8];
pub const SPECIAL_S_NULL: i32 = 0;
pub const SPECIAL_S_NORMAL: i32 = 1;
pub const SPECIAL_S_DARK: i32 = 2;
pub const SPECIAL_S_DARK_ACTIVE: i32 = 3;
pub static mut FINAL_EFFECT:[i32;8] = [-1; 8];

pub static mut BAT_INPUT_X:[f32;8] = [0.0; 8];
pub static mut BAT_INPUT_Y:[f32;8] = [0.0; 8];
pub static mut BAT_DEGREE:[f32;8] = [0.0; 8];
pub static mut BAT_EXIT:[bool;8] = [false; 8];
pub static mut BAT_EXIT_FRAME:[i32;8] = [0; 8];

pub static mut DIVE_TARGET:[i32;8] = [0; 8];

pub unsafe fn reset(boma: *mut BattleObjectModuleAccessor){
    let entry = get_entry_from_boma(boma) as usize;
    META_FRAME[entry]=0;
    META_WHIFF[entry]=false;
    META_HEALED[entry]=false;
    META_EFFECT[entry] = -1;
    META_ATTEMPTS[entry] =0;

    BAT_EXIT[entry]=false;
    BAT_EXIT_FRAME[entry] = 0;
    
    SPECIAL_S_AERIAL[entry]=true;
    SPECIAL_S_TYPE[entry]=0;
    SPECIAL_HI_TYPE[entry]=0;
    FINAL_EFFECT[entry] = -1;
}

pub unsafe fn meta_start(boma: *mut BattleObjectModuleAccessor) {
    let entry = get_entry_from_boma(boma) as usize;
    let mut penalty = META_ATTEMPTS[entry] * 90;
    META_FRAME[entry]= (META_MAX-penalty).max(600);
    META_ATTEMPTS[entry]=0;
}

pub unsafe fn meta_swordattack_status(boma: *mut BattleObjectModuleAccessor){

}
pub unsafe fn meta_swordattack(boma: *mut BattleObjectModuleAccessor) -> bool {
    let status = StatusModule::status_kind(boma);
    let entry = get_entry_from_boma(boma) as usize;
    let mut swordAttack = [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_100,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].contains(&status);
    if (!swordAttack)
    {
        let motion = MotionModule::motion_kind(boma);
        if (status == *FIGHTER_STATUS_KIND_ATTACK_AIR)
        {
            swordAttack = motion == Hash40::new("attack_air_b").hash
            || motion == Hash40::new("attack_air_f").hash
            || motion == Hash40::new("attack_air_f_hi").hash
            || motion == Hash40::new("attack_air_f_lw").hash;
        }
        else
        {
            swordAttack = motion == Hash40::new("attack_13").hash
            || motion == Hash40::new("attack_100").hash
            || motion == Hash40::new("attack_100_end").hash;
        }
        
    }
    return META_FRAME[entry]>0 && swordAttack;
}

pub unsafe fn meta_loss(boma: *mut BattleObjectModuleAccessor,amount: i32) {
    let amount = -(amount.abs());
    let entry = get_entry_from_boma(boma) as usize;

    if META_FRAME[entry]>0
    {
        META_FRAME[entry]= (META_FRAME[entry]-amount).max(0);
        if !EffectModule::is_exist_common(boma, Hash40::new("sys_hit_curse"))
        {
            EffectModule::req_follow(boma, Hash40::new("sys_hit_curse"),Hash40::new("rot"), &Vector3f::zero(), &Vector3f::zero(), 1.25, true, 0, 0, 0, 0, 0, false, false) as u64;
        }
    }
}
pub unsafe fn meta_heal(boma: *mut BattleObjectModuleAccessor,damageDealt: f32) {

    if DamageModule::damage(boma,0) <=0.0 {return;}
    let damageDealt = (damageDealt.abs());
    let entry = get_entry_from_boma(boma) as usize;
    //let entry = get_entry_from_boma(boma) as usize;
    if meta_swordattack(boma)
    {
        let fighter = get_fighter_common_from_accessor(boma);
        GetVar::set_bool(boma, &mut vars::META_HEALED,true);
        if (damageDealt<=3.0 && !(*boma).is_status(*FIGHTER_STATUS_KIND_ATTACK_100)) {return; }
        let heal = damageDealt/-7.5;
        DamageModule::heal(boma,heal,0);
        println!("Richter healed {}",heal);
        PLAY_SE(fighter, Hash40::new("se_common_lifeup"));

        if (!EffectModule::is_exist_common(boma, Hash40::new("sys_badge")))
        {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_badge"), Hash40::new("hip"), 0,0,0,0,0,0, 1.00, true);
            LAST_EFFECT_SET_RATE(fighter,2.5);
        }
    }
}

pub unsafe fn meta_is_active(boma: *mut BattleObjectModuleAccessor) -> bool {
    let entry = get_entry_from_boma(boma) as usize;
    return META_FRAME[entry]>0;
}