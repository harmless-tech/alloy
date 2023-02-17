# Allot

Allot is an in-progress, highly experimental language runtime.

### Install and Run

First install it using cargo install:
```shell
cargo install allot@0.0.2-alpha
```

Then to see options:
```shell
allot --help
```

### Making programs

Right now the only way to make programs for Allot is to use allot_asm. However, allot_asm is currently very janky 
(since it is basically just a text representation of allot bytecode) and does not currently come with any documentation.
There are examples you can make sense of though under crates/allot_asm/programs.
