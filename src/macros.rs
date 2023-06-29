/// Asserts that two vectors containing HashMaps are equal.
#[macro_export]
macro_rules! assert_vec_hashmap_eq {
    ($a:expr, $b:expr) => {{
        let mut a_sorted: Vec<_> = $a
            .iter()
            .map(|h| {
                let mut items: Vec<_> = h.iter().collect();
                items.sort();
                items
            })
            .collect();

        let mut b_sorted: Vec<_> = $b
            .iter()
            .map(|h| {
                let mut items: Vec<_> = h.iter().collect();
                items.sort();
                items
            })
            .collect();

        a_sorted.sort();
        b_sorted.sort();

        assert_eq!(a_sorted, b_sorted, "Vectors of HashMaps are not equal");
    }};
}
