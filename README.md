# numru

![ci](https://img.shields.io/github/actions/workflow/status/mjovanc/numru/ci.yml?branch=main)
![crates.io](https://img.shields.io/crates/v/numru.svg)
[![documentation](https://img.shields.io/badge/docs-numru-blue?logo=rust)](https://docs.rs/numru/latest/)

A high-performance scientific computation library written in Rust.

## Get Started

This getting started guide might change and should not be a source of absolute truth. Check the unit tests if you want to stay up to date with how things should be done. Some APIs will definetely be changed in the future.

```toml
[dependencies]
numru = "0.1.0"
```

And a simple code:

```rust
fn main() {
  let mut a = array![1, 2, 3];
  a.zeros();
  println!("{:?}", a);
}
```

## Features

Numru will offer a variety of different numerical operations and data types. It is intended to be a fundamental library for scientific computing with Rust.

### Supported Data Types

- i64
- f64

### Supported Operations (planned)

Note that currently we only show the numru equivalents as the ones that are planned. They do not exist yet.

| Operation            | NumPy Equivalent                     | Numru Equivalent              |
|----------------------|-------------------------------------|----------------------------------|
| Create Array         | `np.array([2, 3])`                  | `array![1, 2, 3]`              |
| Zeros Array         | `np.zeros((2, 3))`                  | `zeros!(3, 3)` or `a.zeros()`             |
| Ones Array          | `np.ones((2, 3))`                   | `ones!(3, 3)` or `a.ones()`              |
| Arange             | `np.arange(start, stop, step)`      | `arange!(start, stop, step)` |
| Linspace           | `np.linspace(start, stop, num)`     | 🚧 |
| Mean               | `np.mean(a)`                        | `a.mean()`                  |
| Min                | `np.min(a)`                         | `a.min()`                    |
| Max                | `np.max(a)`                         | `a.max()`                    |
| Exp                | `np.exp(a)`                         | 🚧                    |
| Log                | `np.log(a)`                         | 🚧                    |
| Sigmoid            | `1 / (1 + np.exp(-a))`              | 🚧                |
| Dot Product        | `np.dot(a, b)`                      | `dot!(a, b)`                 |
| Reshape           | `a.reshape((2, 3))`                 | `a.reshape(4, 3)`             |
| Concatenate        | `np.concatenate([a,b], axis=0)`     | 🚧 |
| Element-wise Add   | `a + b`                             | `a + b`                          |
| Element-wise Sub   | `a - b`                             | `a - b`                         |
| Element-wise Mul   | `a * b`                             | `a * b`                          |
| Element-wise Div   | `a / b`                             | `a / b`                         |
| Fancy Indexing     | `np.ones((2,3))[0, :]`             | 🚧         |
| Fancy Flipping     | `np.array([1,2,3])[::-1]`          | 🚧      |

## License

The MIT License.
