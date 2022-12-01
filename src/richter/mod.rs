use super::*;
mod jab;
mod backair;
mod upair;
mod downair;
mod forwardtilt;
mod forwardair;
mod forwardsmash;
mod neutralspecial;
mod throws;
mod opff;
mod status;

pub fn install() {
    jab::install();
    forwardtilt::install();
    forwardsmash::install();
    forwardair::install();
    backair::install();
    upair::install();
    downair::install();
    neutralspecial::install();
    throws::install();
    opff::install();
    status::install();
}
