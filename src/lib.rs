#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]
#![deny(deprecated)]
#![allow(unused)]

pub mod richter;
pub mod axe;
pub mod whip;
pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;

use smash::{
    lib::{
        L2CValue,
        LuaConst,
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;


#[skyline::main(name = "smashline_alucard")]
pub fn main() {
    println!("[smashline_alucard::main] Who calls me?");
    richter::install();
    axe::install();
    whip::install();
    unsafe{
	skyline::nn::ro::LookupSymbol(&mut FIGHTER_CUTIN_MANAGER_ADDR, c_str!("_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"));
    }
}