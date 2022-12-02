use super::*;
mod backair;
mod downtilt;

pub fn install() {
    backair::install();
    downtilt::install();
}