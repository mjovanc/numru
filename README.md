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
Check the unit tests and in `examples` if you want to stay up to date with how things should be done. Some APIs will most likely be changed in the future.

```toml
[dependencies]
numru = "0.2.0"
```

And a simple code:

```rust
use numru::arr;
use std::f64::consts::{E, PI, TAU};

fn main() {
    let a = arr![42, -17, 256, 3, 99, -8];
    println!("a.shape() = {:?}", a.shape());
    a.visualize().execute();

    let b = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
    println!("\nb.shape() = {:?}", b.shape());
    b.visualize()
        .decimal_points(1)
        .execute();

    let c = arr![
        [[101, 202, 303], [404, 505, 606]],
        [[-707, -808, -909], [111, 222, 333]]
    ];
    println!("\nc.shape() = {:?}", c.shape());
    c.visualize().execute();
}
```

Output of the code above:

```shell
a.shape() = Shape=Ix { dims: [6] }
[42, -17, 256, 3, 99, -8]

b.shape() = Shape=Ix { dims: [3, 3] }
[
   [6.283185307179586, -3.141592653589793, 1.61 ]
   [2.718281828459045, 0.98              , -7.42]
   [4.67             , -0.45             , 8.88 ]
]

c.shape() = Shape=Ix { dims: [2, 2, 3] }
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

### Supported Operations

Note that currently we only show the numru equivalents as the ones that are planned. They do not exist yet.

| **Operation**          | **Type**              | **NumPy Equivalent**             | **Numru Equivalent**             |
|--------------------|-------------------|-----------------------------|------------------------------|
| Create Array   | Array Creation    | `np.array([1, 2, 3])`       | `arr![1, 2, 3]`             |
| Zeros Array    | Array Creation    | `np.zeros((3,3))`           | `zeros!(i64, 3, 3)`         |
| Ones Array     | Array Creation    | `np.ones((3,3))`            | `zeros!(i64, 3, 3)`                           |
| Arange         | Array Creation    | `np.arange(start, stop, step)` | 🚧                       |
| Linspace       | Array Creation    | `np.linspace(start, stop, num)` | 🚧                      |
| Mean          | Reduction         | `np.mean(a)`                | 🚧        |
| Min           | Reduction         | `np.min(a)`                 | `a.min().compute()`         |
| Max           | Reduction         | `np.max(a)`                 | `a.max().compute()`         |
| Dot Product    | Linear Algebra    | `np.dot(a, b)`              | 🚧                           |
| Reshape        | Manipulation      | `a.reshape((4, 3, 3))`      | 🚧                           |
| Concatenate    | Manipulation      | `np.concatenate([a, b], axis=0)` | 🚧                   |
| Element-wise Add | Element-wise Ops | `a + b`                     | 🚧                           |
| Element-wise Sub | Element-wise Ops | `a - b`                     | 🚧                           |
| Element-wise Mul | Element-wise Ops | `a * b`                     | 🚧                           |
| Element-wise Div | Element-wise Ops | `a / b`                     | 🚧                           |

### Utility Features

These utility features help with visualization, debugging, array exploration and more.

| **Feature**              | **Type**               | **Numru**                                      | **Description**                                      |
|----------------------|-------------------|--------------------------------------------|--------------------------------------------------|
| Visualization    | Visualization   | `a.visualize().execute()`                 | Print an array in a human-readable format       |
| Shape Inspection | Introspection     | `a.shape()`                               | Get the shape of the array                      |
| Data Type Check  | Introspection     | `a.dtype()`                            | Retrieve the data type of the array             |

## License

The MIT License.
