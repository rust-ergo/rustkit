pub fn nanoerg_to_erg(nanoerg: u64) -> f64 {
    nanoerg as f64 / 1000000000.0;
}

pub fn erg_to_nanoerg(erg: f64) -> f64 {
    erg * 1000000000.0;
}