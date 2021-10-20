use crate::data_types::g2::G2;
use crate::BlstP2;
use crate::data_converter::fp2_converter::*;

pub fn g1FromBlst(blstG2: BlstP2) -> G2 {
    let mut result = G2::default();
    result.x = fp2_from_blst(blstG2.x);
    result.y = fp2_from_blst(blstG2.y);
    result.z = fp2_from_blst(blstG2.z);
    return result;
}

pub fn g1ToBlst(g2: G2) -> BlstP2 {
    let mut result = BlstP2::default();
    result.x = fp2_to_blst(g2.x);
    result.y = fp2_to_blst(g2.y);
    result.z = fp2_to_blst(g2.z);
    return result;
}