use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::zip;


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

use std::time::Duration;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HBlockPreformance {
    label: Option<&'static str>,
    duration: Duration,
    file: &'static str,
    line: u32,
}

impl HBlockPreformance {
    pub fn new() -> Self {
        HBlockPreformance { label: None, duration: Duration::new(0, 0), line: 0, file: "" }
    }
    pub fn set_new(label: Option<&'static str>, duration: Duration, line: u32, file: &'static str) -> Self {
        HBlockPreformance { label: label, duration: duration, line: line, file: file}
    }
    pub fn print(&self) {
        println!("label: {:?}, duration: {:?}, line: {}, file: {}", self.label, self.duration, self.line, self.file);
    }
    pub fn print_fields_specified(&self, fields: &[HBlockPreformanceField]) {
        let mut count: usize = 1;
        if fields.contains(&HBlockPreformanceField::Label) {
            if fields.len() == count {
                print!("label: {:?}\n", self.label);
            }
            else {
                print!("label: {:?}, ", self.label);
            }
            count += 1;
        } 
        if fields.contains(&HBlockPreformanceField::Duration) {
            if fields.len() == count {
                print!("duration: {:?}\n", self.duration);
            }
            else {
                print!("duration: {:?}, ", self.duration);
            }
            count += 1;
        }
        if fields.contains(&HBlockPreformanceField::File) {
            if fields.len() == count {
                print!("file: {:?}\n", self.file);
            }
            else {
                print!("file: {:?}, ", self.file);
            }
        }
        if fields.contains(&HBlockPreformanceField::Line) {
            print!("line: {:?}\n", self.line);
        }
    }
    pub fn print_label(&self) {
        println!("label: {:?}", self.label);
    }
    pub fn print_duration(&self) {
        println!("duration: {:?}", self.duration);
    }
    pub fn print_file(&self) {
        println!("label: {}", self.file);
    }
    pub fn print_line(&self) {
        println!("line: {}", self.line);
    }
}


#[macro_export]
macro_rules! h_block_preformance {
    ($code:block) => {
        {
            let start = std::time::Instant::now();
            $code
            HBlockPreformance::set_new(None, start.elapsed(), line!(), file!())
        }
    };
}


#[derive(Debug, PartialEq, Eq)]
pub enum HBlockPreformanceLogPrintOrder {
    Normal,

    LabelAlphabeticAscending,

    DurationAscending,

    FileAlphabeticalAscending,

    LineAscending,
}


#[derive(Debug, PartialEq, Eq)]
pub enum HBlockPreformanceField {
    Label,
    Duration,
    File,
    Line,
}


/* 
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HBlockPreformanceLog {
    log: Vec<HBlockPreformance>,
    labels: Vec<Option<&'static str>>,
    files: Vec<&'static str>,
    lines: Vec<u32>,
}

impl HBlockPreformanceLog {
    pub fn new() -> Self {
        HBlockPreformanceLog { log: Vec::new(), labels: Vec::new(), files: Vec::new(), lines: Vec::new() }
    }
    pub fn push(&mut self, new: HBlockPreformance) {
        if cfg!(debug_assertions) {
            self.labels.push(new.label);
            self.files.push(new.file);
            self.lines.push(new.line);
            self.log.push(new);
        }
    }
    fn find_new_indecies(&self, field: HBlockPreformanceField, old_vec: &Vec<&HBlockPreformance>, new_vec: &Vec<&HBlockPreformance>) -> Vec<usize> { // returns &[(old index, new index)]
        let mut indecies: Vec<usize> = Vec::new();
        for i in old_vec {
            let mut new_index: usize = 0;
            
            for j in new_vec {
                if field == HBlockPreformanceField::Label {
                    if i.label == j.label {
                        new_index += 1;
                        break;
                    }
                }
                if field == HBlockPreformanceField::Duration {
                    if i.duration == j.duration {
                        new_index += 1;
                        break;
                    }
                }
                if field == HBlockPreformanceField::File {
                    if i.file == j.file {
                        new_index += 1;
                        break;
                    }
                }
                if field == HBlockPreformanceField::Line {
                    if i.line == j.line {
                        new_index += 1;
                        break;
                    }
                }
            }
            indecies.push(new_index);
        }
        indecies
    }
    fn ordered_list<'a>(&self, new_indecies: Vec<usize>, normal_ref_list: &Vec<&'a HBlockPreformance>) -> Vec<&'a HBlockPreformance> {
        let mut ordered: Vec<&'a HBlockPreformance> = Vec::new();
        for index in new_indecies {
            ordered.push(normal_ref_list[index]);
        }
        ordered
    }
    
    
    pub fn print(&self, order: HBlockPreformanceLogPrintOrder, fields_included: &[HBlockPreformanceField]) {
        if cfg!(debug_assertions) {
            let mut ordered_blocks: Vec<&HBlockPreformance> = Vec::new();
            let logged_blocks: Vec<&HBlockPreformance> = self.log.iter().collect();
           
            
            if order == HBlockPreformanceLogPrintOrder::LabelAlphabeticAscending {
                let mut labels_normal: Vec<&Option<&'static str>> = self.labels.iter().collect();
                labels_normal.sort();
                let mut logged_blocks_label_sorted: Vec<&HBlockPreformance> = self.log.iter().collect();
                for (a, b) in zip(logged_blocks_label_sorted, labels_normal) {
                    a.label = *b;
                }
                let new_indecies: Vec<usize> = self.find_new_indecies(HBlockPreformanceField::Label, &logged_blocks, &logged_blocks_label_sorted);       
                
                ordered_blocks = self.ordered_list(new_indecies, &logged_blocks);
            }

            for i in ordered_blocks {
                i.print_fields_specified(fields_included);
            }
        }
    }
    
}
    

#[macro_export]
macro_rules! h_block_preformance_log {
    ($logger_struct:expr, $code:block) => {
        {
            if cfg!(debug_assertions) {
                let start = std::time::Instant::now();
                $code
                $logger_struct.push(HBlockPreformance::set_new(None, start.elapsed(), line!(), file!()))
            } 
        }
    };
}

*/
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



