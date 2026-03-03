use core::panic;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;


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


/// This trait converts a list of keys and a list of values into a HashMap. 
/// The keys and values must implement the Eq, Hash, and Clone traits to be stored in a HashMap. 
/// The function will return a HashMap where each key from the list of keys is associated with the corresponding value 
/// from the list of values. For example, if the list of keys is [1, 2, 3] and the list of values is ["a", "b", "c"], 
/// the resulting HashMap will contain the pairs (1, "a"), (2, "b"), and (3, "c"). The function will panic 
/// if the lengths of the keys and values lists do not match, or if there are duplicate keys in the list of keys.
/// Example usage:
/// let keys = vec![1, 2, 3];
/// let values = vec!["a", "b", "c"];
/// let map = keys.h_list_to_hashmap(&values);
/// The result will be a HashMap containing the pairs (1, "a"), (2, "b"), and (3, "c"),
///  because each key from the list of keys is associated with the corresponding value from the list of values.

pub trait ListToHashMap<V> {
    type Key;
    type Value;

    fn h_list_to_hashmap(&self, values: &[V]) -> HashMap<Self::Key, Self::Value>
    where
        Self::Key: Eq + Hash + Clone,
        Self::Value: Clone;
}


impl<K, V> ListToHashMap<V> for [K]
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    type Key = K;
    type Value = V;

    fn h_list_to_hashmap(&self, values: &[V]) -> HashMap<Self::Key, Self::Value> {
        let keys: &[K] = self;

        if keys.len() != values.len() {
            panic!("Keys and values must have the same length");
        }

        let mut set = HashSet::new();
        let mut map = HashMap::new();

        // Check for duplicate keys
        for key in keys.iter() {
            if !set.insert(key.clone()) {
                panic!("Duplicate key found in h_vector_to_hashmap");
            }
        }

        // Build the HashMap
        for i in 0..keys.len() {
            map.insert(keys[i].clone(), values[i].clone());
        }

        map
    }
}


/// this trait converts a list of values into a HashSet. 
/// The values must implement the Eq, Hash, and Clone traits to be stored in a HashSet.
/// The function will return a HashSet containing all the unique values from the list. 
/// For example, if the list contains ["a", "b", "a", "c"],
/// the resulting HashSet will contain "a", "b", and "c", because those are the unique values in the list.
/// Example usage:
/// let values = vec!["a", "b", "a", "c"];
/// let value_set = values.h_list_to_hashset();
/// The result will be a HashSet containing "a", "b", and "c", because those are the unique values in the list.
/// The function will panic if there are duplicate values in the list, as HashSet only stores unique values.
/// For example, if the list contains ["a", "b", "a"], the function will panic with the message "Duplicate value found in h_list_to_hashset" because "a" is duplicated in the list of values.

pub trait ListToHashSet {
    type Item;

    fn h_list_to_hashset(&self) -> HashSet<Self::Item>
    where
        Self::Item: Eq + Hash + Clone;
}

impl<T> ListToHashSet for [T]
where
    T: Eq + Hash + Clone,
{
    type Item = T;

    fn h_list_to_hashset(&self) -> HashSet<Self::Item> {
        let mut set = HashSet::new();
        for item in self.iter() {
            set.insert(item.clone());
        }
        set
    }
}



pub trait Tof64 {
    fn h_f64(&self) -> f64;
}

impl<T> Tof64 for T 
where 
    T: Copy + Into<f64>,
{
    fn h_f64(&self) -> f64 {
        (*self).into()
    }
}

pub trait Toi32 {
    fn h_i32(&self) -> i32;
}

impl<T> Toi32 for T 
where 
    T: Copy + Into<i32>,
{
    fn h_i32(&self) -> i32 {
        (*self).into()
    }
}

pub trait ToVecf64 {
    fn h_to_vec_f64(&self) -> Vec<f64>;
}

impl<T> ToVecf64 for Vec<T> 
where 
    T: Copy + Into<f64>,
{
    fn h_to_vec_f64(&self) -> Vec<f64> {
        let mut new_vec: Vec<f64> = Vec::new();
        for i in self {
            new_vec.push((*i).into());
        }
        return new_vec;
    }
}

pub trait ToVeci32 {
    fn h_to_vec_i32(&self) -> Vec<i32>;
}

impl<T> ToVeci32 for Vec<T> 
where 
    T: Copy + Into<i32>,
{
    fn h_to_vec_i32(&self) -> Vec<i32> {
        let mut new_vec: Vec<i32> = Vec::new();
        for i in self {
            new_vec.push((*i).into());
        }
        return new_vec;
    }
}