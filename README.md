## Bevy Jam #6
Our submission for [Bevy Jam #6](https://itch.io/jam/bevy-jam-6).

## Development
This assumes you have Rust, Cargo etc. setup already. If you haven't done so, visit the [Rust website](https://www.rust-lang.org/tools/install).

### Setup
Run the following command to speed up builds (apparently)
```
cp .cargo/config_fast_builds.toml .cargo/config.toml
```

### Bevy CLI
I recommend installing [bevy_cli](https://github.com/TheBevyFlock/bevy_cli), some of the scripts look pretty handy.
Louis will probably make a VSCode Task Runner thing so he has buttons to click eventually.

Building & running for the web:
```
bevy run web --open
```

Building and run binary locally:
```
bevy run
```

Build only (this is helpful on its own to fix compilation issues):
```
bevy build
```

### Even faster feedback for testing?
Apparently this [tool](https://github.com/TheBevyFlock/bevy_simple_subsecond_system) hotpatches functions to test on the
spot, but I haven't looked into it yet.

### Testing
We don't test here. But if it would be easier to work on a feature with tests, see `cargo test`.

### Release
This repo has Github Actions integration that builds for release on most platforms on new tags (I think). Probably
should test this not the night before.
