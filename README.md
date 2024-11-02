# Montgomery Curve Group Law Implementation in Rust

This Rust project implements the group law operations for points on a Montgomery curve. Montgomery curves are useful in cryptography due to their efficiency in certain operations, particularly scalar multiplication. This implementation focuses on basic point operations over a Montgomery curve, such as point addition, point doubling, and inverses.
## Overview

Montgomery curves are a type of elliptic curve with the general form:
$B \cdot y^2 = x^3 + A \cdot x^2 + x$
where $A$ and $B$ are constants defining the curve. This project uses the `num-bigint` and `num-traits` crates to handle large integer arithmetic for elliptic curve operations on points defined over finite fields.

## Montgomery Curves

A Montgomery curve has the form $By^2 = x^3 + A \cdot x^2 + x $ modulo $p$ where **$A$** and **$B$** are constants specific to the Montgomery curve.

The code handles the group law on Montgomery curves, specifically:
- **Point Addition**: Adds two distinct points on the curve.
- **Point Doubling**: Doubles a point on the curve.
- **Point Inversion**: Finds the inverse of a point with respect to the curve group law.

## Project Structure

The main struct and functions include:
- **`Point` Struct**: Represents a point on the curve, with coordinates `(x, y)` or a point at infinity.
- **`Point::add`**: Adds two points using Montgomery's addition law.
- **`Point::double`**: Doubles a point according to the Montgomery curve group law.
- **`Point::inverse`**: Computes the inverse of a point.
- **`Point::mod_inverse`**: Helper function for calculating modular inverses using the Extended Euclidean Algorithm.

## Usage

### Prerequisites

Ensure you have Rust installed. You can install it from [rustup](https://rustup.rs/).

Add the required dependencies to your `Cargo.toml`:
```toml
[dependencies]
num-bigint = "0.4"
num-traits = "0.2"
```
## Running the Code
To run the code, clone the repository and execute the following command:
```
cargo run
```
## Code Explanation
- **Creating a Point**: Define a point on the curve.
```
let p = Point::new(BigInt::from(12), BigInt::from(6));
let q = Point::new(BigInt::from(5), BigInt::from(5));
```
- **Point Addition**: Add two points `P` and `Q` on the curve.
```
let sum = p.add(&q, &a, &b, &modulo);
println!("P + Q = {:?}", sum);
```
- **Point Doubling**: Double the point `P` on the curve.
```
let doubled = p.double(&a, &b, &modulo);
println!("2P = {:?}", doubled);
```
- **Inverse of a Point**: Compute the inverse of point `P`.
```
let inverse = p.inverse(&modulo);
println!("-P = {:?}", inverse);
```
## Example Output
Expected output when running the code:
```
P + Q = Point { x: Some(15), y: Some(13) }
2P = Point { x: Some(15), y: Some(4) }
-P = Point { x: Some(12), y: Some(11) }
