// Implement a ChainMap as in Python -
// https://blog.florimondmanca.com/a-practical-usage-of-chainmap-in-python

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

type ChainMapType<K, V> = HashMap<K, V>;

pub struct ChainMap<K, V> {
    maps: Vec<ChainMapType<K, V>>,
}

impl<K: Hash + Eq + Clone, V: Clone> ChainMap<K, V> {
    pub fn get(&self, k: &K) -> Option<&V> {
        // Check whether an element is found in any one of the maps
        for m in &self.maps {
            let r = m.get(k);
            if r.is_some() {
                return r;
            }
        }
        None
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if self.maps.len() == 0 {
            None
        } else {
            self.maps[0].insert(k, v)
        }
    }

    pub fn is_empty(&self) -> bool {
        for m in &self.maps {
            if !m.is_empty() {
                return false;
            }
        }
        true
    }

    pub fn new(maps: Vec<ChainMapType<K, V>>) -> Self {
        ChainMap { maps }
    }

    // Had to add Clone due to this method - is it worth it, or is this the best
    // way to do this?
    pub fn parents(&self) -> Option<Self> {
        if self.maps.len() > 0 {
            let (ps, _) = self.maps.split_at(self.maps.len() - 1);
            Some(ChainMap { maps: ps.to_vec() })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_chainmap_insert() {
        {
            let m = HashMap::<i32, String>::new();
            let mut cmap = ChainMap::new(vec![m]);
            assert_eq!(cmap.insert(1, "one".to_string()), None);
        }

        {
            let mut m = HashMap::<i32, String>::new();
            m.insert(1, "one".to_string());

            let mut cmap = ChainMap::new(vec![m]);
            assert_eq!(cmap.insert(1, "two".to_string()), Some("one".to_string()));
        }
    }

    #[test]
    fn test_chainmap_get() {
        let mut m = HashMap::<i32, String>::new();
        m.insert(1, "one".to_string());
        m.insert(2, "two".to_string());
        m.insert(3, "three".to_string());

        let cmap = ChainMap::new(vec![m]);

        assert_eq!(cmap.get(&3), Some(&"three".to_string()));
        assert_eq!(cmap.get(&2), Some(&"two".to_string()));
        assert_eq!(cmap.get(&1), Some(&"one".to_string()));

        assert_eq!(cmap.get(&5), None);
    }

    #[test]
    fn test_chainmap_parents() {
        let mut m1 = HashMap::<i32, String>::new();
        m1.insert(1, "one".to_string());
        m1.insert(2, "two".to_string());
        m1.insert(3, "three".to_string());

        let mut m2 = HashMap::<i32, String>::new();
        m2.insert(11, "eleven".to_string());
        m2.insert(22, "twenty two".to_string());
        m2.insert(33, "thirty three".to_string());

        let cmap = ChainMap::new(vec![m1, m2]);

        assert!(cmap.parents().is_some());

        let cmap2 = cmap.parents().unwrap();
        assert!(cmap2.parents().is_some());

        assert_eq!(cmap2.get(&1), Some(&"one".to_string()));
        assert_eq!(cmap2.get(&2), Some(&"two".to_string()));
        assert_eq!(cmap2.get(&3), Some(&"three".to_string()));
        assert_eq!(cmap2.get(&22), None);

        let cmap3 = cmap2.parents().unwrap();
        assert!(cmap3.parents().is_none());
    }

    #[test]
    fn test_chainmap_empty() {
        // Non-empty one
        {
            let mut m: HashMap<i32, String> = HashMap::new();
            m.insert(3, "three".to_string());

            let cmap = ChainMap::new(vec![m]);
            assert!(!cmap.is_empty());
        }

        // Empty one
        {
            let cmap = ChainMap::new(vec![HashMap::<i32, String>::new()]);
            assert!(cmap.is_empty());
        }
    }
}
