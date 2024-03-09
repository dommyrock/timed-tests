#[allow(unused_imports)]
use rand::Rng;
#[allow(unused_imports)]
use std::collections::{BTreeMap, HashMap, HashSet};
use macros::timed;

//Used only for docs / Not actually needed (we parse Time variants as path in proc macro handler)
pub enum Time {
    Second,
    Mili,
    Micro,
    Nano,
}

#[timed(Time::Micro)]
#[test]
fn t1() {
    let mut a: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let b: Vec<i32> = vec![2, 5, 6];

    let start_idx = a.len() - b.len();
    let mut idx = 0;

    for i in start_idx..start_idx + b.len() {
        a[i] = b[idx];
        idx += 1;
    }

    a.sort();
    assert_eq!(a, [1, 2, 2, 3, 5, 6])
}

#[test]
#[timed("Micro")]
fn t2() {
    let mut a: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let b: Vec<i32> = vec![2, 5, 6];

    let start_idx = a.len() - b.len();

    for i in 0..b.len() {
        unsafe {
            *a.get_unchecked_mut(start_idx + i) = *b.get_unchecked(i);
        }
    }

    a.sort_unstable();
    assert_eq!(a, [1, 2, 2, 3, 5, 6]);
}

#[test]
#[timed(Time::Nano)]
fn t3_stack() {
    let mut a: [i32; 6] = [1, 2, 3, 0, 0, 0];
    let b: [i32; 3] = [2, 5, 6];

    let start_idx = a.len() - b.len();

    for i in 0..b.len() {
        unsafe {
            *a.get_unchecked_mut(start_idx + i) = *b.get_unchecked(i);
        }
    }
    a.sort();
    assert_eq!(a, [1, 2, 2, 3, 5, 6]);
}

#[timed(Micro)]
#[test]
fn t4_btree_map_into() {
    let a: Vec<i32> = vec![1, 3, 4, 1, 2, 3, 1, 1, 2, 3, 4];
    let r = get_2d(a);

    fn get_2d(input: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: BTreeMap<u8, HashSet<i32>> = BTreeMap::new();

        let mut key = 1;
        for n in input {
            if map.values_mut().all(|v| !v.insert(n)) {
                map.insert(key, [n].into());
                key += 1;
            }
        }

        map.into_values().map(|v| v.into_iter().collect()).collect()
    }
    assert_eq!(
        r.len(),
        vec![vec![2, 1, 3, 4], vec![3, 1], vec![1], vec![1, 2, 3, 4]].len()
    )
}

#[timed(Micro)]
#[test]
fn t5_btree_map_arrays() {
    #[inline(always)]
    fn get_2d(input: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
    
        for n in input {
            let mut found = false;
            for vec in &mut result {
                if !vec.contains(&n) {
                    vec.push(n);
                    found = true;
                    break;
                }
            }
            if !found {
                result.push(vec![n]);
            }
        }
    
        result
    }

    let r = get_2d( vec![1, 3, 4, 1, 2, 3, 1, 1, 2, 3, 4]);
    assert_eq!(
        r.len(),
        vec![vec![2, 1, 3, 4], vec![3, 1], vec![1], vec![1, 2, 3, 4]].len()
    )
}

#[timed(Second)]
#[test]
fn t6_big_rand_input() {
        #[inline(always)]
        fn get_2d(input: Vec<i32>) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::new();
        
            for n in input {
                let mut found = false;
                for vec in &mut result {
                    if !vec.contains(&n) {
                        vec.push(n);
                        found = true;
                        break;
                    }
                }
                if !found {
                    result.push(vec![n]);
                }
            }
        
            result
        }
        
        fn generate_random_array(length: usize, range_from: i32, range_to: i32) -> Vec<i32> {
            let mut rng = rand::thread_rng();
            (0..length).map(|_| rng.gen_range(range_from..=range_to)).collect()
        }
    
        
        let rand= generate_random_array(10_000, 1, 100);
    
        let _r = get_2d( rand);
}


