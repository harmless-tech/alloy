# allot_asm

Currently, a heavy work in-progress. Check back later for documentation and a better README.

Right now allot_asm has a very basic lexer and parser which are closely tied to how Allot represents instructions internally.

### Small example
```
mov r9 str(This is a string!)
call (println) ; Parentheses are required.
mov r9 none()
; This is a comment
exit i32(11)
```
