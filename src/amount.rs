#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount {
    sats: u64,
}

impl Amount {
    pub fn from_sats(sats: u64) -> Self {
        Self { sats }
    }

    pub fn to_sats(self) -> u64 {
        todo!("return sats exactly")
    }
}
