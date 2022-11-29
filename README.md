# Multi-threaded-Map-Reduce-in-Rust
Using multi-threading in Rust to implement the [MapReduce](https://en.wikipedia.org/wiki/MapReduce) algorithm


# Requirements
- Installation of Rust 

---

## Building
- Run `cargo build`

---

## Testing
- Run `cargo run x y`, where x is the number of partitions and y is the size of the whole dataset
- The data will be output to the terminal that the above command was executed in

---

## Explanation
This program will generate a large dataset of random numbers and then based on a partition size, perform the MapReduce algorithm over the partitions.
This is meant to be a naive implementation without considerations for fault tolerance or scheduling procedures for induvidual "`mappers`". The program uses multithreading to 
process the generated dataset in parallel and then eventually combine the data in the end. There will be a single-threaded output just above the multi-threaded output 

---