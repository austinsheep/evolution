Evolution
=========
Copyright (c) 2019 Austin Baugh

> `Evolution` is a 2D simulation based on Daniel Shiffman's coding challenge of implementing [Evolutionary Steering Behaviors](https://youtu.be/flxOkx0yLrY).

- This simulation is written in the Rust programming language, and uses the [ggez](https://github.com/ggez/ggez) crate.
- This simulation consists of fish who improve their survival behaviors through the use of a genetic algorithm.
    - The fish should be inclined to be attracted to food/prey and repelled by predators
    - The fish should also develop perception radii for prey and predators respectively that works mostly optimally.
- The fish reproduce asexually by cloning themselves, with the possibility of their gene's mutating (mutation rate).
- The speed of the fish is inversely proportional to their size.
    - Therefore larger fish will be slower, while smaller fish will be faster.
- Since there are predators in this simulation, their are different groups of fish, depending on their link in the food chain.
    - Different fish groups have different ranges of sizes/speeds.
    - E.g. The highest-levelled group in the food chain (the group of predators that can't don't have predators) will have the largest size options and smallest speed options.
    - E.g. The lowest-levelled group in the food chain (the group of fish that isn't predators) will have the smallest size options and largest speed options.

## Build and Run
This program requires `cargo`, which can be installed [here](https://rustup.rs).
I advise using the `--release` argument when building or running to achieve a higher FPS.
- Build: `cargo build --release`
- Run: `cargo run --release`

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
