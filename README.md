Evolution
=========
Copyright (c) 2019 Austin Baugh

> `Evolution` is a 2D simulation based on Daniel Shiffman's coding challenge of implementing [Evolutionary Steering Behaviors](https://youtu.be/flxOkx0yLrY).

- This simulation is written in the Rust programming language, and uses the [ggez](https://github.com/ggez/ggez) crate.
- This simulation consists of fish who improve their "foraging" and "avoiding" behaviors through the use of a genetic algorithm.
    - By "foraging" it is meant that fish will be inclined to consume food.
    - By "avoiding" it is meant that fish will be inclined to avoid predators.

## Build and Run
This program requires `cargo`, which can be installed [here](https://rustup.rs).
- Build: `cargo build`
- Run: `cargo run`

## [Documentation](https://web.pdx.edu/~abaugh/doc/evolution)

## Features
- [x] Animate fish in a throughful way
- [x] Add seek behavior to fish
- [x] Add a declining health attribute to fish
- [x] Add food entities
    - [x] Have fish seek food
    - [x] Have eating food replenish lost health
- [x] Implement predators
    - [x] Have fish avoid predators
    - [x] Have fish seek prey
    - [x] Have eating prey replenish lost health
- [x] Incorperate cloning
    - [x] Apply a mutation rate in the cloning function

## License
This program is licensed under the [MIT License](https://github.com/austinsheep/evolution/blob/master/LICENSE)

## Additional References
- [The Nature of Code by Daniel Shiffman | Chapter 6. Autonomous Agents](https://natureofcode.com/book/chapter-6-autonomous-agents/)
- [Steering Behaviors For Autonomous Characters by Craig Reynolds](http://www.red3d.com/cwr/steer/)
