# Lucid Raycaster

A simple raycasting engine that I originally wrote back in college (2017) and recently decided to rewrite in Rust for no
particular reason. The original version was written in Java, but let's be honest for a bit and admit that 
absolutely no one likes Java. Perhaps not even Java developers themselves. So here we are.

## What's New?

The Rust rewrite adds some new features to the original:
- Proper movement and collision detection (We're big boys now)
- Enemy AI with patrol paths and combat behavior
- Projectile system with particle effects
- TOML-based map format for easy modding

## Building & Running

# Clone the repo

# Build it
cargo build --release

# Run it
cargo run --release
```

## Controls
- WASD: Move around
- Mouse: Look around
- Left Click: Shoot
- Escape: Toggle mouse capture

## "But what's the purpose of this project?"

If you're asking that question, you're clearly thinking too hard about it.
1. It's fun
2. Rust
3. See points 1 and 2

## Future Plans

I honestly don't know if I'll do anything else with this. Maybe I'll add more features if inspiration strikes, or maybe it'll just sit here as a testament to the fact that everything will eventually be rewritten in <insert flavor of the month language here>.

## License

Do whatever you want with it. If you somehow find a use for this code, you're probably a genius or insane. Either way, I respect that.