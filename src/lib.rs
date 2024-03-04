use macros::timed;

pub enum Time {
    Second,
    Mili,
    Micro,
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
#[timed(Time::Nano)]
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

//ADD CRITERION TO GROUP ALL THE TESTS IN CURRENT FILE INTO GROUP ... 