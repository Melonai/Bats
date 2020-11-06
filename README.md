# 🦇 Bats

Bats is a **dynamic**, **functional** and **structural** programming language.

### So why Bats?

Bats is supposed to be easy and fun to work with, because that is what programming should be.
It is nowhere near complete nor production-ready, but could be fun to write small projects in, so decide for yourself.

## Specification

### Comments

Comments in Bats start with the `#` token.
A comment without a matching closing comment token lasts the entire line.

### Variables

Variables are declared and initialized in a single line. A variable is never without a value.
Every variable is bound to the block it was declared in, once the block ends the variable falls out of scope.

```
a = "This is a string value"
b = nil
```

### The `nil` value

The `nil` value in Bats denotes the absence of a value.
It is only ever equal to itself.

### Expressions and Statements

Bats does not differentiate between expressions and statements, as all statements in Bats are simply expressions with a return value of `nil` or `never`.
Blocks in Bats are also expressions.

### Blocks

Everything in Bats takes place somewhere in a block.
A block in Bats can be denoted either with whitespace notation or with inline notation through the `|` token.
Both notations can be mixed easily, and the only difference between them is stylistic.
Because of this the inline notation is usually recommended for shorter blocks.

```
# Raw blocks which get executed immediately

a =
    # Whitespace notation
    -> "Hello!"

a = | # Inline notation # -> "Hello!" |

a = | # Mixed notation
    -> "Hello!" |
```

You may have noticed the use of the `->` token in our example using blocks. This is how you return values from a block in Bats. 
A `->` token means that the immediate block in which this statement is inside of will evaluate to the value after the token. 
To access blocks higher in scope, such as within nested blocks, the token can be stacked.

```
f = do
    a = | -> -> "The function f will return this string!" |
```

Before the last example every block we've shown was a raw block.
The interpreter executes raw blocks immediately and evaluates them to whichever value the block returned.
Other block types can be denoted using **block headers**.
One such block header is the `do` header, which creates a function from the block bound to it.
Block headers mutate the way a block is executed and return types other than the type specified in the block.
This is used for patterns like functions, branching logic and more.