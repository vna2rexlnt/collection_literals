use std::collections::{HashMap, HashSet};
use collections_literals::{collection, hash};

#[test]
fn it_should_create_defaults() {
    let tested_set: HashSet<u8> = hash! {};
    let desired_set: HashSet<u8> = collection! {};
    assert_eq!(tested_set, desired_set);

    let tested_set: HashMap<u8, bool> = hash! {};
    let desired_set: HashMap<u8, bool> = collection! {};
    assert_eq!(tested_set, desired_set);
}

#[test]
fn it_should_properly_create_sets() {
    let tested_set = hash! {1, 2, 3, 4, 5, 6, 7, 8, 9};
    let desired_set: HashSet<i32> = collection! {1, 2, 3, 4, 5, 6, 7, 8, 9};
    assert_eq!(tested_set, desired_set);

    let tested_set = hash! {1, 1, 1, 8, 8, 8};
    let desired_set: HashSet<i32> = collection! {1, 8};
    assert_eq!(tested_set, desired_set);
}

fn is_prime<T: Into<i64>>(number: T) -> bool
{
    let number = number.into();
    let float = number as f64;
    let s = float.sqrt().trunc() as i64;

    for d in 2..=s {
        if number % d == 0 {
            return false;
        }
    }

    true
}

#[test]
fn it_should_properly_create_maps() {
    let tested_map = hash! {
        1 => is_prime(1),
        2 => is_prime(2),
        3 => is_prime(3),
        4 => is_prime(4),
        5 => is_prime(5),
        6 => is_prime(6),
        7 => is_prime(7),
        8 => is_prime(8),
        9 => is_prime(9),
    };
    let desired_map: HashMap<i32, bool> = collection! {
        1 => is_prime(1),
        2 => is_prime(2),
        3 => is_prime(3),
        4 => is_prime(4),
        5 => is_prime(5),
        6 => is_prime(6),
        7 => is_prime(7),
        8 => is_prime(8),
        9 => is_prime(9),
    };
    assert_eq!(tested_map, desired_map);

    let tested_map = hash! {
        1 => true,
        1 => true,
        3 => true,
        3 => false,
        5 => true,
        5 => false,
        7 => true,
        7 => false,
        9 => false,
    };
    let desired_map: HashMap<u8, bool> = collection! {
        1 => true,
        3 => false,
        5 => false,
        7 => false,
        9 => false,
    };
    assert_eq!(tested_map, desired_map);
}