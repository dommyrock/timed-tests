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

#### NOTE:
I could use file modules to devide test types **"mod array_ops{}"**, **"mod search{}"** ...
<br/>
Than use criterion to do different group plots...
