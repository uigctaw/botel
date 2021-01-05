# botel

Botel programming language specification and its interpreter implemented
in Rust.

This is just a personal project to learn Rust.

## Goals

To make it really hard to break stuff from within a botel program
and to make a developer feel the pain of side effects and nudge them to
record them (think event sourcing). Furthermore the syntax should be clean
and easy to understand. "Executable pseudo code" of sorts.

## Language characteristics

- Almost no side effects.\*
- No mutable state.
- First class functions.
- Dynamic, strong typing with no coercion.
- No reflection.
- No metaprogramming.
- Very high level. No concept of threads or anything related to the
underlying physical machine. Well... That's probably too optimistic
and at least some interpreter flags or annotations will be necessary
for practical reasons. But I'd really like to avoid putting it
in the core language.

### \* Almost no side effects

Botel is intended to be benign: almost completely unable
to interact with its environment. No reading or writing to files or other IO.

The interpreter will not be a standalone binary. It will be a library
to be used from within a "controller" Rust program.
Something like this (pseudo code):

```
input = get_initial_input()
while input:
    output = botel_program.run(input)
    input = do_something_with_output(output)
assert botel_program.is_finished()
```

The `botel_program` object will hold state. It will be a coroutine
yielding partial results so that the controller program can do side effects
(such as query a database) and fetch the results back to the `botel_program`,
so that it can continue execution.

## Interesting points

Functions are not called. They simply exist in data context and
represent immutable values.

## Extended Backus-Naur Form grammar

TBD

## Examples

```
x = "Hello world"
```

```
times_two = 2*n
x = times_two(3)  # will result in error, double partially applied below
n = 10
```

```
times_two = 2*n
times_four = times_two(times_two)
should_be_8 = times_four(2)
#!assert should_be_8 == 8
four_times_square  = times_two * times_two
should_be_16 = four_times_square(2)
#!assert should_be_16 == 16
```
