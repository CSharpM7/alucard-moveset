use super::*;
mod jab;
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
mod throws;
mod taunts;
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
    downtilt::install();
    downsmash::install();
    upsmash::install();
    uptilt::install();
    neutralspecial::install();
    sidespecial::install();
    neutralair::install();
    taunts::install();
    throws::install();
    opff::install();
    status::install();
}
