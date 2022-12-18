use super::*;

pub const LENGTH: f32 = 15.0;

pub static mut META_HEALED:[bool;8] = [false; 8];
pub static mut META_WHIFF:[bool;8] = [false; 8];
pub static mut META_FRAME:[i32;8] = [0; 8];
pub static mut META_EFFECT:[i32;8] = [-1; 8];
pub static mut META_ATTEMPTS:[i32;8] = [-1; 8];
pub const META_MAX: i32 = 1500;
pub const META_PUNISH: i32 = 60;

pub static mut SPECIAL_N_SPAWN:[i32;8] = [0; 8];

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
}

pub unsafe fn meta_start(boma: *mut BattleObjectModuleAccessor) {
    let entry = get_entry_from_boma(boma) as usize;
    let mut penalty = META_ATTEMPTS[entry] * 90;
    META_FRAME[entry]= (META_MAX-penalty).max(600);
    META_ATTEMPTS[entry]=0;
}

pub unsafe fn meta_is_active(boma: *mut BattleObjectModuleAccessor) -> bool {
    let entry = get_entry_from_boma(boma) as usize;
    return META_FRAME[entry]>0;
}