# Rust 6502

A (newbie-level) interpretation of the MOS-6502 instruction set in Rust, to learn more Rust

## current test results:

```
Running tests/tests.rs (/mnt/c/Users/xbibl/Dropbox/code/mos6502/rust/rust6502/target/debug/deps/tests-b372e1ae11ac2269)

running 13 tests
test tests::LDA_absolute ... ok
test tests::LDA_absolute_X ... ok
test tests::LDA_absolute_Y ... ok
test tests::LDA_absolute_X_cross_page ... ok
test tests::LDA_zero_page ... ok
test tests::LDA_zero_page_X ... ok
test tests::LDA_absolute_Y_cross_page ... ok
test tests::LDA_immediate ... ok
test tests::LDA_zero_page_X_wraparound ... ok
test tests::cpu_init_pc ... ok
test tests::cpu_init_sp ... ok
test tests::mem_test_basic ... ok
test tests::setup_debug_autotest ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## why do this?

- learning to play better with memory
- learning to play better with rust
- learning how to build a basic cpu emulator so I can build bigger things in the future!


## why the mos 6502?

- it's a good simple instruction set
- a variant of it was used on the NES!

## basic implementation concept

- I've been working on this in C++ alongside various online resources, and so the code is sort of refactored and ported into Rust
- Generally, I **try to write the tests first** and then try to write the implementation on failing tests.
- In this case, since there is a bit of porting, the test / instruction relationship is somewhat unaligned currently compared to the C++ version. I'll be updating this soon!

## what to do (in rust)?

- [x] LDA instruction implementation
- [ ] LDA instruction testing
- [ ] LDX instruction implementation
- [ ] LDX instruction testing
- [ ] LDY instruction implementation 
- [ ] LDY instruction testing
