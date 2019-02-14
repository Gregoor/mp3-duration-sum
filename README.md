# mp3-duration-sum ![](https://img.shields.io/crates/v/mp3-duration-sum.svg?style=flat)

```bash
cargo install -f mp3-duration-sum
mp3-duration-sum some_dir
```

Parallelized counting of duration of all `.mp3` files in the given directory.

All the heavy lifting is done by:
- [mp3-duration](https://github.com/agersant/mp3-duration)
- [rayon](https://github.com/rayon-rs/rayon)
