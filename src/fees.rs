#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeeRate {
    pub sat_per_vb: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxSizeEstimate {
    pub vbytes: u64,
}

pub fn fee(size: TxSizeEstimate, rate: FeeRate) -> u64 {
    size.vbytes * rate.sat_per_vb
}
