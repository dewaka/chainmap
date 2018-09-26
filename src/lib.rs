use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

type ChainMapType<K, V> = HashMap<K, V>;

pub struct ChainMap<K, V> {
    maps: Vec<ChainMapType<K, V>>,
}

impl<K: Hash + Eq, V> ChainMap<K, V> {
    /// Returns the first element found in the maps chain or None
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

    /// Inserts an element to the first map of the chain if maps are not empty,
    /// and returns the current value in the case where the key exists, or
    /// otherwise None
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if self.maps.len() == 0 {
            None
        } else {
            self.maps[0].insert(k, v)
        }
    }

    /// Returns whether the ChainMap is empty or not
    pub fn is_empty(&self) -> bool {
        for m in &self.maps {
            if !m.is_empty() {
                return false;
            }
        }
        true
    }

    /// Constructs a ChainMap with given vector of sub-maps
    pub fn new(maps: Vec<ChainMapType<K, V>>) -> Self {
        ChainMap { maps }
    }

    /// Constructs an empty ChainMap
    pub fn empty() -> Self {
        ChainMap::new(vec![])
    }

    /// Adds a map to the end of the maps chain
    pub fn add_map(&mut self, m: ChainMapType<K, V>) -> &mut Self {
        self.maps.push(m);
        self
    }
}

impl<K: Hash + Eq + Clone, V: Clone> ChainMap<K, V> {
    /// Returns a single consolidated map
    pub fn to_map(&self) -> ChainMapType<K, V> {
        let mut combined_map = ChainMapType::new();

        for m in &self.maps {
            for (k, v) in m {
                combined_map.insert(k.clone(), v.clone());
            }
        }

        combined_map
    }

    /// Returns an optional ChainMap with just the end of maps removed, if the
    /// current ChainMap is not empty. Otherwise returns None
    pub fn parents(&self) -> Option<Self> {
        if self.maps.len() > 0 {
            let (ps, _) = self.maps.split_at(self.maps.len() - 1);
            Some(ChainMap { maps: ps.to_vec() })
        } else {
            None
        }
    }

    /// Returns an optional ChainMap with just head of maps removed, if the
    /// current ChainMap is not empty. Otherwise returns None
    pub fn children(&self) -> Option<Self> {
        if self.maps.len() > 0 {
            let (_, cs) = self.maps.split_at(1);
            Some(ChainMap { maps: cs.to_vec() })
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
    fn test_chainmap_children() {
        let mut m1 = HashMap::<i32, String>::new();
        m1.insert(1, "one".to_string());
        m1.insert(2, "two".to_string());
        m1.insert(3, "three".to_string());

        let mut m2 = HashMap::<i32, String>::new();
        m2.insert(11, "eleven".to_string());
        m2.insert(22, "twenty two".to_string());
        m2.insert(33, "thirty three".to_string());

        let cmap = ChainMap::new(vec![m1, m2]);

        assert!(cmap.children().is_some());

        let cmap2 = cmap.children().unwrap();
        assert!(cmap2.children().is_some());

        assert_eq!(cmap2.get(&11), Some(&"eleven".to_string()));
        assert_eq!(cmap2.get(&22), Some(&"twenty two".to_string()));
        assert_eq!(cmap2.get(&33), Some(&"thirty three".to_string()));
        assert_eq!(cmap2.get(&1), None);
        assert_eq!(cmap2.get(&2), None);

        let cmap3 = cmap2.children().unwrap();
        assert!(cmap3.children().is_none());
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

    #[test]
    fn test_chainmap_add_map() {
        let mut m: HashMap<i32, String> = HashMap::new();
        m.insert(3, "three".to_string());

        let mut cmap: ChainMap<i32, String> = ChainMap::empty();
        assert_eq!(cmap.get(&3), None);

        cmap.add_map(m);
        assert_eq!(cmap.get(&3), Some(&"three".to_string()));
    }

    #[test]
    fn test_chainmap_to_map() {
        let mut m1 = HashMap::<i32, String>::new();
        m1.insert(1, "one".to_string());
        m1.insert(2, "two".to_string());
        m1.insert(3, "three".to_string());

        let mut m2 = HashMap::<i32, String>::new();
        m2.insert(11, "eleven".to_string());
        m2.insert(22, "twenty two".to_string());
        m2.insert(33, "thirty three".to_string());

        let cmap = ChainMap::new(vec![m1, m2]);

        let combined_map = cmap.to_map();

        assert_eq!(combined_map.get(&1), Some(&"one".to_string()));
        assert_eq!(combined_map.get(&2), Some(&"two".to_string()));
        assert_eq!(combined_map.get(&3), Some(&"three".to_string()));
        assert_eq!(combined_map.get(&11), Some(&"eleven".to_string()));
        assert_eq!(combined_map.get(&22), Some(&"twenty two".to_string()));
        assert_eq!(combined_map.get(&33), Some(&"thirty three".to_string()));
    }
}
