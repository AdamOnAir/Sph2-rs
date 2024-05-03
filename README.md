# Sph2-rs

## What is it
A new version of Sph-rs that uses a module.

## Technical Badges
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

## Run
Use the following :
```bash
cargo run 2.25
>>> The perimeter of the circle is: 14.137166941154069
>>> The area of the circle is: 15.904312808798327
>>> The surface area of the sphere is: 63.61725123519331
>>> The volume of the sphere is: 47.71293842639498
```
Replace 2.25 with the wanted value.

You could also include "calcul.rs" as it is a mod and call it.
For example :
```rust
let p = calcul::perimetre_cercle(rayon);
print!("Surface area of the circle is : {}", p);
```
