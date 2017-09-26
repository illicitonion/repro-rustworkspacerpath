# Rust workspace + rpath issue

Running with a `[workspace]` entry in the Cargo.toml forces a runtime requirement on @rpath/stdlib, but means that my non-root tests run:

```
$ rm -rf target ; cargo build && otool -L target/debug/libtoplevel.dylib
   Compiling dep v0.1.0 (file:///Users/dwagnerhall/src/github.com/illicitonion/repro-rustworkspacerpath/dep)
   Compiling toplevel v0.1.0 (file:///Users/dwagnerhall/src/github.com/illicitonion/repro-rustworkspacerpath)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54 secs
target/debug/libtoplevel.dylib:
	/Users/dwagnerhall/src/github.com/illicitonion/repro-rustworkspacerpath/target/debug/deps/libtoplevel.dylib (compatibility version 0.0.0, current version 0.0.0)
	@rpath/libstd-85bc071b5d81d134.dylib (compatibility version 0.0.0, current version 0.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1238.60.2)
	/usr/lib/libresolv.9.dylib (compatibility version 1.0.0, current version 1.0.0)
$ cargo test --all
   Compiling dep v0.1.0 (file:///Users/dwagnerhall/src/github.com/illicitonion/repro-rustworkspacerpath/dep)
   Compiling toplevel v0.1.0 (file:///Users/dwagnerhall/src/github.com/illicitonion/repro-rustworkspacerpath)
    Finished dev [unoptimized + debuginfo] target(s) in 0.66 secs
     Running target/debug/deps/dep-30a1304fc03c1a40

running 1 test
test tests::j ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/toplevel-fe032189c7aabc0d

running 1 test
test tests::i ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests dep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Removing the `[workspace]` entry in Cargo.toml doesn't show such a requirement, but also stops my non-root tests from running:

```
$ sed -e '/\[workspace\]/d' -i '' Cargo.toml
$ rm -rf target ; cargo build && otool -L target/debug/libtoplevel.dylib
   Compiling dep v0.1.0 (file:///Users/dwagnerhall/src/github.com/illicitonion/repro-rustworkspacerpath/dep)
   Compiling toplevel v0.1.0 (file:///Users/dwagnerhall/src/github.com/illicitonion/repro-rustworkspacerpath)
    Finished dev [unoptimized + debuginfo] target(s) in 0.82 secs
target/debug/libtoplevel.dylib:
	/Users/dwagnerhall/src/github.com/illicitonion/repro-rustworkspacerpath/target/debug/deps/libtoplevel.dylib (compatibility version 0.0.0, current version 0.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1238.60.2)
	/usr/lib/libresolv.9.dylib (compatibility version 1.0.0, current version 1.0.0)
$ cargo test --all
   Compiling toplevel v0.1.0 (file:///Users/dwagnerhall/src/github.com/illicitonion/repro-rustworkspacerpath)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61 secs
     Running target/debug/deps/toplevel-fe032189c7aabc0d

running 1 test
test tests::i ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
