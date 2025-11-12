# eval

## eval — a tiny math expression evaluator (friendly docs)

Hey — this crate evaluates simple math strings like "3 + 4 * 2" and gives you a number.

I kept the API tiny on purpose — it's all about basic arithmetic and easy-to-understand behaviour.

What it does
- Parses and evaluates expressions with `+`, `-`, `*`, `x` (for multiply), and `/`.
- Respects multiplication/division precedence over addition/subtraction.
- Supports unary `+` and `-` (so stuff like `-3 + 2` works).
- Ignores spaces, so ` 3 x 4 + 2 ` is fine.

Quick examples

```rust
use eval::eval;

assert_eq!(eval("22+22-22x22"), -440.0); // 22 + 22 - 22*22
assert_eq!(eval("-1 + 2"), 1.0);
assert_eq!(eval(" 3 x 4 + 2 "), 14.0);
```

Why use this? (short)
- It's tiny and dependency-free.
- Good for small tools, learning, or quick scripting where you need to evaluate math typed as text.

Limitations / caveats
- No parentheses support (yet).
- Non-numeric or malformed input will panic — keep inputs trusted or pre-validated.
- Works with decimal points (floats), e.g. `3.5 * 2`.

Style note (cuz you asked)
- The docs here use plain words and a relaxed, conversational tone — cuz docs shouldn't read like a robot.

Want more features?
- I can add parentheses, return Result types for safer error handling, or add extra operators. Tell me which and I'll wire it up.

Enjoy! — simple, small, and practical.
