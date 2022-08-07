// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut hash = HashMap::new();
    for s in magazine {
        let counter = hash.entry(s).or_insert(0);
        *counter += 1
    }

    for w in note {
        let v = hash.entry(w).or_insert(0);
        *v -= 1;
        if *v < 0 {
            return false
        }
    }
    return true

}
