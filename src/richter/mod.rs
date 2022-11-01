use super::*;
mod jab;
mod backair;
mod upair;
mod forwardair;
mod opff;
mod status;

pub fn install() {
    jab::install();
    forwardair::install();
    backair::install();
    upair::install();
    opff::install();
    status::install();
}
