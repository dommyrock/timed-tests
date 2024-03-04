### Test

```shell
cargo test --lib t1     #target t1 test
```

#### --nocapture
- propagate test println to std_out
**All**

```shell
cargo test -- --nocapture
```

**Target**

```shell
cargo test --lib t1 -- --nocapture
```

---

### Expand

**All**

```shell
cargo expand --lib --tests
```

**Target**

```shell
cargo expand --lib --tests t1
```

#### tests module

- mod tests {}

```rust
#[cfg(test)]
  mod tests {
  #[timed(Time::Nano)]
  #[test]
  fn t1() {...}
```
**#[cfg(test)]** ONLY needed if i have inner module tests {}!
```shell
cargo expand --lib --tests tests::t2
```

<br/>

-  No mod tests{}

```rust
#[timed(Time::Nano)]
#[test]
fn t1() {} ...
```

```shell
cargo expand --lib --tests t2
```

### Expanded "t1"test 
```rust
fn t1() {
    let start = std::time::Instant::now();
    let result = {
        let mut a: Vec<i32> = <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([1, 2, 3, 0, 0, 0]),
        );
        let b: Vec<i32> = <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([2, 5, 6]),
        );
        let start_idx = a.len() - b.len();
        let mut idx = 0;
        for i in start_idx..start_idx + b.len() {
            a[i] = b[idx];
            idx += 1;
        }
        a.sort();
        match (&a, &[1, 2, 2, 3, 5, 6]) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    let elapsed = start.elapsed();
    {
        ::std::io::_print(
            format_args!("Time elapsed: {0:.2} {1}",elapsed.as_secs_f32() * 1_000_000.0,"µs (microseconds) | ",
            ),
        );
    };
    {
        ::std::io::_print(format_args!("{0} {1:?}\n", elapsed.as_nanos(), "nanoseconds"),);
    };
    result
}
```

### Expanded "t3_stack" test (stack alocated vec for faster access)
```rust
fn t3_stack() {
    let start = std::time::Instant::now();
    let result = {
        let mut a: [i32; 6] = [1, 2, 3, 0, 0, 0];
        let b: [i32; 3] = [2, 5, 6];
        let start_idx = a.len() - b.len();
        for i in 0..b.len() {
            unsafe {
                *a.get_unchecked_mut(start_idx + i) = *b.get_unchecked(i);
            }
        }
        a.sort();
        match (&a, &[1, 2, 2, 3, 5, 6]) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    let elapsed = start.elapsed();
    {
        ::std::io::_print(
            format_args!("Time elapsed: {0:.2} {1}",elapsed.as_secs_f32() * 1_000_000.0,"µs (microseconds) | ",
            ),
        );
    };
    {
        ::std::io::_print(format_args!("{0} {1:?}\n", elapsed.as_micros(), "microseconds"),);
    };
    result
}
```

#### NOTE:
I could use file modules to devide test types **"mod array_ops{}"**, **"mod search{}"** ...
<br/>
Than use criterion to do different group plots...
