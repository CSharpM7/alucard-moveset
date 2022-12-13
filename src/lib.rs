#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]
#![deny(deprecated)]
#![allow(unused)]

pub mod richter;
pub mod whip;
pub mod axe;
pub mod cross;
pub mod update;

use skyline_web::*;

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
    util::*,
    ext::*
};
use smashline::*;


#[skyline::main(name = "smashline_alucard")]
pub fn main() {
    println!("[smashline_alucard::main] Who calls me?");
    update::install();
    richter::install();
    whip::install();
    axe::install();
    cross::install();
}