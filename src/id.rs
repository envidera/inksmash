use std::sync::atomic::{AtomicUsize, Ordering};

/// automatic incremental id
pub fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

//=====================================================================
// UNIT TESTS
//=====================================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_id() {
        for i in 1..50 {
            assert_eq!(get_id(), i);
        }
    }
}
