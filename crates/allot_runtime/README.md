# allot_runtime

Currently, a heavy work in-progress. Check back later for documentation and a better README.

### Some Plans

- Split bytecode into its own crate.
- Use fixed-map or phf for binding runtime functions. (Call Instruction)
- Use Rc for ThreadHandle to allow Type to be Cloneable and Copiable.
