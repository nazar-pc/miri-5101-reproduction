<https://github.com/rust-lang/miri/issues/5101>

```bash
RUSTFLAGS="-Znext-solver=globally" MIRIFLAGS="-Znext-solver=globally" cargo miri test -Ztarget-applies-to-host -Zhost-config --config 'host.rustflags=["-Znext-solver=globally"]' --lib
```
