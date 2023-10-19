use std::collections::HashMap;

pub fn prefix_all_keys<V>(map: &mut HashMap<String, V>, prefix: &str) -> HashMap<String, V> {
    map
        .drain()
        .map(|(key, value)| (format!("{}{}", prefix, key), value))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_all_keys() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), 42);
        map.insert("key2".to_string(), 55);

        map = prefix_all_keys(&mut map, "prefix_");

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("prefix_key1"), Some(&42));
        assert_eq!(map.get("prefix_key2"), Some(&55));
    }

    #[test]
    fn test_empty_map() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map = prefix_all_keys(&mut map, "prefix_");
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_empty_prefix() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), 42);

        map = prefix_all_keys(&mut map, "");

        assert_eq!(map.len(), 1);
        assert_eq!(map.get("key1"), Some(&42));
    }
}
