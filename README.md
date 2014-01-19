# The Matasano Crypto Challenges

### Requires rust 0.9

```
git clone https://github.com/mozilla/rust.git && cd rust
./configure && make && make install
```

### Check answers are correct

```bash
rustpkg test set1/problem1
rustpkg test set1/problem2
rustpkg test set1/problem3
rustpkg test set1/problem4
```

### See solutions

```bash
rustpkg install set1/problem3 && ./bin/problem3
rustpkg install set1/problem4 && ./bin/problem4
```