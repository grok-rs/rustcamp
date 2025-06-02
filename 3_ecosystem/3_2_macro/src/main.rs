// Declarative macro implementation
#[macro_export]
macro_rules! btreemap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::BTreeMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

// Test suite
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_declarative_macro() {
        let map: BTreeMap<&str, i32> = btreemap!("a" => 1, "b" => 2, "c" => 3);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
        assert_eq!(map.get("c"), Some(&3));
    }

    #[test]
    fn test_empty_map() {
        let map: BTreeMap<&str, i32> = btreemap!();
        assert!(map.is_empty());
    }
}
