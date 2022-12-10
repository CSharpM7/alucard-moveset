use super::*;
mod jab;
mod dashattack;
mod backair;
mod upair;
mod downair;
mod downtilt;
mod downsmash;
mod uptilt;
mod upsmash;
mod forwardtilt;
mod forwardair;
mod forwardsmash;
mod neutralair;
mod neutralspecial;
mod sidespecial;
mod upspecial;
mod throws;
mod taunts;
mod opff;
mod status;

pub fn install() {
    jab::install();
    dashattack::install();
    forwardtilt::install();
    forwardsmash::install();
    forwardair::install();
    backair::install();
    upair::install();
    downair::install();
    downtilt::install();
    downsmash::install();
    upsmash::install();
    uptilt::install();
    neutralspecial::install();
    sidespecial::install();
    upspecial::install();
    neutralair::install();
    taunts::install();
    throws::install();
    opff::install();
    status::install();
}
