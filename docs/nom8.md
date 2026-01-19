# nom 8.0 API Detailed Documentation

nom is a Rust-based parser combinators library. Its goal is to provide tools for building safe parsers without sacrificing speed or memory consumption.

In nom 8.0, the most significant change is the shift from **closure-based combinators** to **Trait-based combinators**. It is now recommended to use `parser.parse(input)` instead of `parser(input)`.

---

## 1. Core Concepts

### `IResult<I, O, E>`

The standard return type for nom parsers.

- `I`: Type of remaining input (usually `&[u8]` or `&str`).
- `O`: Type of parsed output.
- `E`: Error type (defaults to [(I, ErrorKind)](cci:1://file:///d:/Users/admin/workspace/nom/examples/json.rs:27:0-36:1)).

```rust
type IResult<I, O, E = (I, ErrorKind)> = Result<(I, O), Err<E>>;
```

### `Parser` Trait

The core of nom 8.0. Almost all parsers implement this interface, allowing chained calls.

- [parse(input)](cci:1://file:///d:/Users/admin/workspace/nom/examples/json.rs:38:0-55:1): Execute parsing.
- `map(f)`: Transform output result.
- `and_then(p2)`: Use first parser's output as second parser's input.

---

## 2. Basic Parsers (Basic Elements)

Used to recognize the lowest-level elements in syntax.

| Function     | Description                                    | Example                                        |
| :----------- | :--------------------------------------------- | :--------------------------------------------- | ------- | ----------------------------------------- |
| `tag`        | Match specific string or byte sequence         | `tag("hello").parse("hello world")`            |
| `char`       | Match single character                         | `char('a').parse("abc")`                       |
| `take`       | Extract specified number of bytes or chars     | `take(4usize).parse("hello")`                  |
| `take_while` | Extract as long as predicate is satisfied      | `take_while(                                   | c: char | c.is_alphabetic()).parse("abc123")` |
| `is_a`       | Match characters contained in given set        | `is_a("abc").parse("aabbccdde")`               |
| `is_not`     | Match characters not contained in given set    | `is_not(" \t").parse("hello world")`           |

---

## 3. Choice Combinators

Used to handle multiple possible parsing paths.

### `alt`

Try a series of parsers, return the first successful result.

```rust
use nom::branch::alt;
use nom::bytes::complete::tag;

let mut parser = alt((tag("abc"), tag("def")));
assert_eq!(parser.parse("abc"), Ok(("", "abc")));
assert_eq!(parser.parse("def"), Ok(("", "def")));
```

---

## 4. Sequence Combinators

Used to combine multiple parsers in sequence.

| Function         | Description                                             | Example                                                      |
| :--------------- | :------------------------------------------------------ | :----------------------------------------------------------- |
| `tuple`          | Execute multiple parsers in sequence, results in tuple  | `tuple((tag("ab"), tag("cd"))).parse("abcd")`                |
| `preceded`       | Match first and discard, return second's result         | `preceded(tag("("), tag("abc")).parse("(abc")`               |
| `terminated`     | Match first and return, match second and discard        | `terminated(tag("abc"), tag(")")).parse("abc)")`             |
| `delimited`      | Match three, discard first and last, return middle      | `delimited(char('('), tag("abc"), char(')')).parse("(abc)")` |
| `pair`           | Match two parsers, return tuple                         | `pair(tag("a"), tag("b")).parse("ab")`                       |
| `separated_pair` | Match two parsers separated by third (discarded)        | `separated_pair(tag("a"), char(','), tag("b")).parse("a,b")` |

---

## 5. Multi Parsers (Repetition)

Used to apply the same parser multiple times.

| Function          | Description                                       | Example                                             |
| :---------------- | :------------------------------------------------ | :-------------------------------------------------- |
| `many0`           | Match 0 or more times, results in `Vec`           | `many0(char('a')).parse("aaa")`                     |
| `many1`           | Match 1 or more times                             | `many1(char('a')).parse("aaa")`                     |
| `separated_list0` | Match separator-delimited list (0 or more times)  | `separated_list0(char(','), digit1).parse("1,2,3")` |
| `count`           | Match exact specified number of times             | `count(char('a'), 3).parse("aaa")`                  |
| `many_till`       | Repeat first parser until second parser succeeds  | `many_till(anychar, tag("end")).parse("abcend")`    |

---

## 6. Modifiers (Transform & Modify)

Used to modify parser behavior or transform its output.

- **`map`**: Transform parsing result.
  ```rust
  map(digit1, |s: &str| s.parse::<u32>().unwrap()).parse("123")
  ```
- **`map_res`**: Transform result, if transform function returns `Result::Err`, parsing fails.
- **`opt`**: Make parser optional (returns `Option<O>`).
- **`cut`**: Error escalation. Once parsing after `cut` fails, no backtracking (from `Error` to `Failure`).
- **`peek`**: Look at input without consuming.
- **`recognize`**: If sub-parser succeeds, return the raw input it consumed.
- **`all_consuming`**: Require parser to consume all input, otherwise error.

---

## 7. Number Parsers

Located in `nom::number::complete` or `nom::number::streaming`.

- **Integers**: `be_u32` (big-endian), `le_i16` (little-endian), `u8`, `i64`, etc.
- **Floating point**: `float`, `double`.
- **Hexadecimal**: `hex_u32`.

---

## 8. Character Parsers

Located in `nom::character::complete`.

- `alpha1` / `alpha0`: Letters.
- `digit1` / `digit0`: Digits.
- `alphanumeric1`: Letters or digits.
- `multispace1` / `multispace0`: Spaces, tabs, newlines.
- `line_ending`: `\n` or `\r\n`.

---

## 9. Error Handling

nom 8.0 recommends using `nom-language` for better error messages.

- **`ErrorKind`**: Basic error enum.
- **`VerboseError`**: (requires `nom-language` crate) Records complete error path and context.
- **`context`**: Add descriptive label to parsing step.

```rust
use nom_language::error::VerboseError;
fn parser(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("my_tag", tag("hello")).parse(i)
}
```

---

## 10. nom 8.0 Migration Notes

1.  **Trait First**: Previously `tag("abc")(input)`, now `tag("abc").parse(input)`.
2.  **Tuple Parameters**: `alt` and `tuple` now accept tuples as parameters, not the previous macros or nesting.
    - Correct: `alt((p1, p2, p3))`
3.  **Module Paths**: `nom::bits` is no longer re-exported at root. Use `nom::bits::complete::tag`.
4.  **Input Trait**: Multiple input-related Traits (like `InputIter`, `Slice`) have been merged into a unified `Input` Trait.

---
