#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount {
    sats: u64,
}

impl Amount {
    pub fn from_sats(sats: u64) -> Self {
        Self { sats }
    }

    pub fn to_sats(self) -> u64 {
        self.sats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_preserves_sats_exactly() {
        let amount = Amount::from_sats(50_000);
        assert_eq!(amount.to_sats(), 50_000);
    }
}
