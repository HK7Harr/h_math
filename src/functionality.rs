use core::panic;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;



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

impl<K, V> ListToHashMap<V> for Vec<K>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    type Key = K;
    type Value = V;

    fn h_list_to_hashmap(&self, values: &[V]) -> HashMap<Self::Key, Self::Value> {
        self.as_slice().h_list_to_hashmap(values)
    }
}

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

impl<T> ListToHashSet for Vec<T>
where
    T: Eq + Hash + Clone,
{
    type Item = T;

    fn h_list_to_hashset(&self) -> HashSet<Self::Item> {
        self.as_slice().h_list_to_hashset()
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