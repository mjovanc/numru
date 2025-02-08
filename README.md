# numru

![ci](https://img.shields.io/github/actions/workflow/status/mjovanc/numru/ci.yml?branch=main)
![crates.io](https://img.shields.io/crates/v/numru.svg)
[![documentation](https://img.shields.io/badge/docs-numru-blue?logo=rust)](https://docs.rs/numru/latest/)

A high-performance scientific computation library written in Rust.

## Motivation

Numru is a scientific computation library that aims to provide a high-performance, easy-to-use, and flexible API for numerical operations.
It is inspired by NumPy, a popular numerical computation library in Python. Numru is designed to be a fundamental library for scientific computing with Rust.

## Get Started

This getting started guide might change and should not be a source of absolute truth.
Check the unit tests if you want to stay up to date with how things should be done. Some APIs will definetely be changed in the future.

```toml
[dependencies]
numru = "0.1.0"
```

And a simple code:

```rust
fn main() {
    let a = arr![42, -17, 256, 3, 99, -8];
    println!("1D Shape={:?}", a.shape());
    a.visualize();

    let b = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
    println!("2D Shape={:?}", b.shape());
    b.visualize();

    let c = arr![
        [[101, 202, 303], [404, 505, 606]],
        [[-707, -808, -909], [111, 222, 333]]
    ];
    println!("3D Shape={:?}", c.shape());
    c.visualize();
}
```

Output of the code above:

```shell
1D Shape=Shape=Ix { dims: [6] }
[42, -17, 256, 3, 99, -8]
2D Shape=Shape=Ix { dims: [3, 3] }
[
   [6.283185307179586, -3.141592653589793, 1.61 ]
   [2.718281828459045, 0.98              , -7.42]
   [4.67             , -0.45             , 8.88 ]
]
3D Shape=Shape=Ix { dims: [2, 2, 3] }
[
   [
      [101 , 202 , 303 ]
      [404 , 505 , 606 ]
   ]
   [
      [-707, -808, -909]
      [111 , 222 , 333 ]
   ]
]
```

## Features

Numru will offer a variety of different numerical operations and data types. It is intended to be a fundamental library for scientific computing with Rust.

### Supported Data Types

- i64
- f64

### Planned Data Types (Future)

- i8, i16, i32, i128
- u8, u16, u32, u64, u128
- f32
- bool
- String, &str

### Supported Operations (planned)

Note that currently we only show the numru equivalents as the ones that are planned. They do not exist yet.

| Operation            | NumPy Equivalent                     | Numru Equivalent             |
|----------------------|-------------------------------------|---------------------------------|
| Create Array         | `np.array([1, 2, 3])` | `arr![1, 2, 3]` |
| Zeros Array         | `np.zeros((3, 3, 3))`                  | 🚧            |
| Ones Array          | `np.ones((3, 3, 3))`                   | 🚧             |
| Arange             | `np.arange(start, stop, step)`      | 🚧 |
| Linspace           | `np.linspace(start, stop, num)`     | 🚧 |
| Mean               | `np.mean(a)`                        | `a.mean()`                 |
| Min                | `np.min(a)`                         | `a.min()`                   |
| Max                | `np.max(a)`                         | `a.max()`                   |
| Exp                | `np.exp(a)`                         | 🚧                   |
| Log                | `np.log(a)`                         | 🚧                   |
| Sigmoid            | `1 / (1 + np.exp(-a))`              | 🚧               |
| Dot Product        | `np.dot(a, b)`                      | 🚧                |
| Reshape           | `a.reshape((4, 3, 3))`                 | 🚧            |
| Concatenate        | `np.concatenate([a, b], axis=0)`     | 🚧 |
| Element-wise Add   | `a + b`                             | 🚧                         |
| Element-wise Sub   | `a - b`                             | 🚧                        |
| Element-wise Mul   | `a * b`                             | 🚧                         |
| Element-wise Div   | `a / b`                             | 🚧                        |
| Fancy Indexing     | `np.ones((3, 3, 3))[0, :]`             | 🚧         |
| Fancy Flipping     | `np.array([1, 2, 3])[::-1]`          | 🚧      |

## License

The MIT License.
