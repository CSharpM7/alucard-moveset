use super::*;
mod jab;
mod backair;
mod upair;
mod forwardtilt;
mod forwardair;
mod forwardsmash;
mod opff;
mod status;

pub fn install() {
    jab::install();
    forwardtilt::install();
    forwardsmash::install();
    forwardair::install();
    backair::install();
    upair::install();
    opff::install();
    status::install();
}
