# MicroStackLang
A minimalistic stack-based language

!This langauge is currently unfinished!

## Usage

Build:
```
rustc main.rs -o main
```

Run:
The code should be stored in a file in the local dicrectory called `in.txt`

Run the following command to execute the program
```
./main
```

## Examples
The plain text explanations at the ends of the lines are ignores (only digits and special characters are valid commands)

Print numbers from 9 to 1
```
7>;             skip to instruction seven at start of line three
<:>;            pushes return address onto value stack
<.              drop return address
10              loop counter
3>;<.           call function in line three to get the current instruction pointer
11+:26+>>0>     loop setup    push address of first and last instruction followed by a zero onto the call stack
  <.            loop start    discard return address from top of call stack
    1-:48+"10"  loop body     subtract one from loop counter print as ascii digit then print newline
  :             loop cleanup  duplicate loop counter
  <<:>^:>       loop cleanup  push copies of the two loop addresses onto the value stack
  >^<^?.>       loop cleanup  swap addresses if loop counter is non zero discard top value
;               end of loop   jump to start or to next instruction
<.              end of loop   discard return address
```

\[TODO\] add other examples

## Virtual Machine

The code runs on a virtual machine with two stacks (value-stack and call-stack) and a region of random access memory.

Random access memory is an extension feature to make writing programs involving lists slightly more convenient,
the language should be Turing complete without the memory operations (There should be a relatively straight forward reduction to Brainfuck, but I have not yet found the time/motivation for a formal proof).

All values in the program are integers of a fixed size, the current implementation uses 16-bit values this may change in a future update.
Reading an uninitialized memory address, or popping from and empty stack should result in the value 0, without generating any error messages.

## Syntax
There are 14 operations:

Command | Name | Operation
--- | --- | ---
(`0`-`9`)* | push int | pushes the integer given as decimal number onto the value stack
`>` | cpushv | move top value from value-stack to call-stack
`<` | vpushc | move top value from call-stack to value-stack
`^` | swap | exchange top two elements of value stack
`:` | dup | duplicate the top element of the value stack
`.` | drop | discard the top element of the value stack
`?` | swap-if | pop the top stack value, if it is non-zero swap the top two stack-values
`;` | cswap-ip | swap the top value of the call-stack with the instruction pointer
`~` | is-negative | pop top value, push 1 if negative and 0 otherwise
`+` | add | add the top two stack values
`-` | sub | subtract the top stack value from the second stack value
`_` | read | read a byte from standard input, push it onto the stack (currently unimplemented)
`"` | write | pop the top stack value, write the lower 8bits to standard output
`@` | mswap | pop -> val , pop -> id , old\_mem = mem\[id\] , mem\[id\] = val , push old\_mem
