use macros::timed;

pub enum Time {
    Second,
    Mili,
    Nano,
}

#[timed(Time::Nano)]
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
fn t2() {
    let mut a: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let b: Vec<i32> = vec![2, 5, 6];
    let mut result = vec![0; a.len() + b.len()];

    let mut result = vec![0; a.len() + b.len()];
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            result[k] = a[i];
            i += 1;
        } else {
            result[k] = b[j];
            j += 1;
        }
        k += 1;
    }

    while i < a.len() {
        result[k] = a[i];
        i += 1;
        k += 1;
    }

    while j < b.len() {
        result[k] = b[j];
        j += 1;
        k += 1;
    }
    a = result;
    assert_eq!(a, [1, 2, 2, 3, 5, 6])
}

//ADD CRITERION TO GROUP ALL THE TESTS IN CURRENT FILE INTO GROUP ... 