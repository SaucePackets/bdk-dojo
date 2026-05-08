pub fn dojo_ready() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_repo_is_ready() {
        assert!(dojo_ready());
    }
}
