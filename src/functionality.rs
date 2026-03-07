use std::collections::{HashMap, HashSet};
use std::hash::Hash;


#[derive(Debug)]
pub enum ListToHashMapError<K> {
    LengthMismatch { keys_len: usize, values_len: usize },
    DuplicateKeys { duplicates: Vec<K> },
}


/// This trait converts the values of a HashMap into a HashSet. The values must implement the Eq, Hash, 
/// and Clone traits to be stored in a HashSet. The function will return a HashSet containing all the unique values 
/// from the HashMap. For example, if the HashMap contains the pairs (1, "a"), (2, "b"), and (3, "a"), 
/// the resulting HashSet will contain "a" and "b", because "a" is duplicated in the values of the HashMap.
/// Example usage:
/// let mut map = HashMap::new();
/// map.insert(1, "a");
/// map.insert(2, "b");
/// map.insert(3, "a");
/// let value_set = map.h_hashmap_values_to_hashset();
/// The result will be a HashSet containing "a" and "b", because those are the unique values in the HashMap.
pub trait HashMapValuesToHashSet<V>
where
    V: Eq + Hash + Clone,
{
    fn h_hashmap_values_to_hashset(&self) -> HashSet<V>;
}

impl<K, V> HashMapValuesToHashSet<V> for HashMap<K, V>
where
    V: Eq + Hash + Clone,
{
    fn h_hashmap_values_to_hashset(&self) -> HashSet<V> {
        let mut set: HashSet<V> = HashSet::new();
        for (_key, value) in self.iter() {
            set.insert(value.clone());
        }
        set
    }
}


/// This trait converts the keys of a HashMap into a HashSet. The keys must implement the Eq, Hash, 
/// and Clone traits to be stored in a HashSet. The function will return a HashSet
/// containing all the unique keys from the HashMap. For example, if the HashMap contains the pairs (1, "a"), (2, "b"), 
/// and (3, "a"),
/// the resulting HashSet will contain 1, 2, and 3, because those are the unique keys in the HashMap.
/// Example usage:
/// let mut map = HashMap::new();
/// map.insert(1, "a");
/// map.insert(2, "b");
/// map.insert(3, "a");
/// let key_set = map.h_hashmap_keys_to_hashset();
/// The result will be a HashSet containing 1, 2, and 3, because those are the unique keys in the HashMap.
pub trait HashMapKeysToHashSet<K>
where
    K: Eq + Hash + Clone,
{
    fn h_hashmap_keys_to_hashset(&self) -> HashSet<K>;
}

impl<K, V> HashMapKeysToHashSet<K> for HashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    fn h_hashmap_keys_to_hashset(&self) -> HashSet<K> {
        let mut set: HashSet<K> = HashSet::new();
        for (key, _value) in self.iter() {
            set.insert(key.clone());
        }
        set
    }
}


/// This function converts a list of keys and a list of values into a HashMap.
/// The keys and values must implement the Eq, Hash, and Clone traits to be stored in a HashMap.
/// The function will return a Result containing a HashMap where each key from the list of keys is associated with the corresponding value
/// from the list of values, or a vector of errors if issues are found (length mismatch or duplicate keys).
/// For example, if the list of keys is [1, 2, 3] and the list of values is ["a", "b", "c"],
/// the resulting HashMap will contain the pairs (1, "a"), (2, "b"), and (3, "c").
/// If lengths don't match or there are duplicate keys, it returns errors.
/// Example usage:
/// let keys = vec![1, 2, 3];
/// let values = vec!["a", "b", "c"];
/// let map = h_list_to_hashmap(&keys, &values);
/// The result will be Ok(HashMap) containing the pairs (1, "a"), (2, "b"), and (3, "c").

pub fn h_list_to_hashmap<K, V>(keys: &[K], values: &[V]) -> Result<HashMap<K, V>, Vec<ListToHashMapError<K>>>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    if keys.len() != values.len() {
        return Err(vec![ListToHashMapError::LengthMismatch {
            keys_len: keys.len(),
            values_len: values.len(),
        }]);
    }

    let mut map = HashMap::new();
    let mut duplicates = Vec::new();

    for (key, value) in keys.iter().zip(values.iter()) {
        if map.contains_key(key) {
            duplicates.push(key.clone());
        } else {
            map.insert(key.clone(), value.clone());
        }
    }

    if !duplicates.is_empty() {
        return Err(vec![ListToHashMapError::DuplicateKeys {
            duplicates,
        }]);
    }

    Ok(map)
}





/// this trait converts a list of values into a HashSet. 
/// The values must implement the Eq, Hash, and Clone traits to be stored in a HashSet.
/// The function will return a tuple of (HashSet containing all the unique values from the list, number of duplicates removed). 
/// For example, if the list contains ["a", "b", "a", "c"],
/// the result will be (HashSet containing "a", "b", "c", 1), because "a" was duplicated.
/// Example usage:
/// let values = vec!["a", "b", "a", "c"];
/// let (value_set, dupes) = values.h_list_to_hashset();
/// value_set will contain "a", "b", "c", and dupes will be 1.

pub trait ListToHashSet {
    type Item;

    fn h_list_to_hashset(&self) -> (HashSet<Self::Item>, usize)
    where
        Self::Item: Eq + Hash + Clone;
}

impl<T> ListToHashSet for [T]
where
    T: Eq + Hash + Clone,
{
    type Item = T;

    fn h_list_to_hashset(&self) -> (HashSet<Self::Item>, usize) {  
        let mut set = HashSet::new();
        let mut duplicates = 0;
        for item in self.iter() {
            if !set.insert(item.clone()) {
                duplicates += 1;
            }
        }
        (set, duplicates)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_hashmap_values_to_hashset() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "a");
        let set = map.h_hashmap_values_to_hashset();
        assert_eq!(set.len(), 2);
        assert!(set.contains("a"));
        assert!(set.contains("b"));
    }

    #[test]
    fn test_hashmap_keys_to_hashset() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "a");
        let set = map.h_hashmap_keys_to_hashset();
        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_h_list_to_hashmap() {
        let keys = vec![1, 2, 3];
        let values = vec!["a", "b", "c"];
        let map = h_list_to_hashmap(&keys, &values).unwrap();
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&2), Some(&"b"));
        assert_eq!(map.get(&3), Some(&"c"));
    }

    #[test]
    fn test_h_list_to_hashmap_length_mismatch() {
        let keys = vec![1, 2];
        let values = vec!["a", "b", "c"];
        let result = h_list_to_hashmap(&keys, &values);
        assert!(result.is_err());
    }

    #[test]
    fn test_h_list_to_hashmap_duplicate_keys() {
        let keys = vec![1, 1, 2];
        let values = vec!["a", "b", "c"];
        let result = h_list_to_hashmap(&keys, &values);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_to_hashset() {
        let list = vec!["a", "b", "a", "c"];
        let (set, dupes) = list.h_list_to_hashset();
        assert_eq!(set.len(), 3);
        assert!(set.contains("a"));
        assert!(set.contains("b"));
        assert!(set.contains("c"));
        assert_eq!(dupes, 1);
    }
}
