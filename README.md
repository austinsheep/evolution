Evolution
=========
Copyright (c) 2019 Austin Baugh

> `Evolution` will be a 2D simulation based on Daniel Shiffman's coding challenge of implementing [Evolutionary Steering Behaviors](https://youtu.be/flxOkx0yLrY).

- This simulation is written in the Rust programming language, and uses the [ggez](https://github.com/ggez/ggez) crate.
- This simulation consists of vehicles who improve their "foraging" behaviors through the use of a genetic algorithm.
    - By "foraging" it is meant that vehicles will be inclined to consume food and avoid poison.

## Build and Run
- Build: `cargo build`
- Run: `cargo run`

## [Documentation](https://web.pdx.edu/~abaugh/doc/evolution)

## Features
- [x] Add seek behavior to vehicles (for the cursor's location)
- [ ] Add a declining health attribute to vehicles
- [ ] Add food entities
    - [ ] Have vehciles seek food
- [ ] Add poison entities
    - [ ] Have vehciles avoid poison
- [ ] Incorperate genetic algorithm
- [ ] Implement predators
    - [ ] Consider removing poison

## License
This program is licensed under the [MIT License](https://github.com/austinsheep/evolution/blob/master/LICENSE)

## Additional References
- [The Nature of Code by Daniel Shiffman | Chapter 6. Autonomous Agents](https://natureofcode.com/book/chapter-6-autonomous-agents/)
- [Steering Behaviors For Autonomous Characters by Craig Reynolds](http://www.red3d.com/cwr/steer/)
