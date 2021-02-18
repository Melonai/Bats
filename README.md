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

Bats does not differentiate between expressions and statements, as all statements in Bats are simply expressions with a return value of `nil`.
Blocks in Bats are also expressions.

### Blocks

Everything in Bats takes place somewhere in a block.
A block in Bats is denoted conventially using an opening `{` token and a matching closing `}`.
Bats does not care about whitespace.

```
# Raw blocks get executed immediately

a = {
    -> "Hello!"
}

b = { -> "Hello again!" }
```

You may have noticed the use of the `->` token in our example using blocks. This is how you return values from a block in Bats.
A `->` token means that the immediate block in which this statement is inside of will evaluate to the value after the token.
To access blocks higher in scope, such as within nested blocks, the token can be stacked.

```
f = do {
        a = { -> -> "The function f will return this string!" }
    }
```

Before the last example every block we've shown was a raw block.
The interpreter executes raw blocks immediately and evaluates them to whichever value the block returned.
Bats is able to defer the execution of blocks until neccessary.
This could be done using the `do`, which creates a function from the block bound to it.

### Names

Name values in Bats are similar to symbols in Ruby and atoms in Elixir, in Bats they have another primary use though:
As Bats aims to have a syntax which is woven dynamically, names are used to express identifiers as values.

```
printer = do :value print(value)
```

Bats will make sure that the `value` identifier is available in the scope of the block bound to `printer`.
For multiple variables you can provide the `do` directive a tuple instead of a single name.

```
adder = do (:a :b) a + b
```

### Tuples

Tuples in Bats are very similar blocks, however they are always evaluated instantly
and the result of every expression in available by it's position.

```
tuple = ("First Value" "Second Value")
print(tuple.1)
```
