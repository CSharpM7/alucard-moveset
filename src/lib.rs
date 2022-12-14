#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]
#![deny(deprecated)]
#![allow(unused)]

pub mod richter;
pub mod whip;
pub mod axe;
pub mod cross;
pub mod coffin;
pub mod vars;
pub use vars::*;
pub mod hook;
pub mod params;

#[cfg(feature = "updater")]
use skyline_web::*;
#[cfg(feature = "updater")]
pub mod update;


use smash::{
    lib::{
        L2CValue,
        LuaConst,
        lua_const::*
    },
    //lib::,
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
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use sharpsmashlinesuite::{
    *,
    util::{
        *,
        self
    },
    ext::*
};
use smashline::*;
use std::convert::TryInto;

#[skyline::main(name = "smashline_alucard")]
pub fn main() {
    println!("[smashline_alucard::main] Who calls me?");
    println!("");
    richter::install();
    whip::install();
    axe::install();
    cross::install();
    coffin::install();
    hook::install();
    params::install();
    #[cfg(feature = "updater")]
    update::install();
    /* 
    #[cfg(feature = "updater")]
    {
        std::thread::Builder::new()
            .stack_size(0x40_0000)
            .spawn(|| {
                update::check_for_updates();
            })
            .unwrap()
            .join();
    }*/
}