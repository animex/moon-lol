---
trigger: always_on
---

# General Code Style Guide

1. Indentation should not exceed 3 levels. Use early return to reduce indentation and avoid nested if statements.
2. If indentation must exceed 3 levels, the code should be split into multiple functions.
3. Comments should be concise. Do not write comments for obvious code.

# Rust

1. Use let-else return or if return to reduce indentation
2. Import at the top of the file. Do not use qualified imports, use `use` statements instead
3. When using debug!, write readable logs, e.g.: write {:?} attacked {:?}, not attacker: {:?} target: {:?}
