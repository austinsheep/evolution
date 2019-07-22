# Evolution
Copyright (c) 2019 Austin Baugh

`Evolution` is a 2D simulation based on Daniel Shiffman's coding challenge of implementing [Evolutionary Steering Behaviors](https://youtu.be/flxOkx0yLrY).

### Similarities to Daniel Shiffman's simulation
- `Evolution` consists of vehicles who improve their "foraging" behaviors through the use of a genetic algorithm.  By "foraging" it is meant that vehicles will be inclined to consume food and avoid poison.

### Differences to Daniel Shiffman's simulation
- `Evolution` is written in the Rust programming language, and uses the [ggez](https://github.com/ggez/ggez) crate.
- Predators are a planned feature, which ultimately will replace the role of poison.

## Build and Run
- Build: `cargo build`
- Run: `cargo run`

## License
This program is licensed under the [MIT License](https://github.com/austinsheep/evolution/blob/master/LICENSE)

## Additional References
- [The Nature of Code by Daniel Shiffman | Chapter 6. Autonomous Agents](https://natureofcode.com/book/chapter-6-autonomous-agents/)
- [Steering Behaviors For Autonomous Characters by Craig Reynolds](http://www.red3d.com/cwr/steer/)
