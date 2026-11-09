# flappy

Flappy bird clone on top of my [renderer](https://github.com/Manny-S-04/renderer). Everything is drawn with renderer's canvas functions (triangles, circles, rects) and shown with `renderer::window`, so it runs on Linux (X11) and Windows.

```sh
cargo run --release
```

It uses renderer as a path dependency at `../GITHUB/renderer`, so that needs to be checked out next to it.

## Controls

- space, up, w, k or click - flap (also starts and restarts)
- p - pause
- r - restart
- esc or q - quit

## Layout

- `src/game.rs` - game state, physics, pipes, collisions. No drawing in here
- `src/draw.rs` - sky, clouds, pipes, ground, bird, score screens
- `src/font.rs` - 3x5 pixel digits for the score
- `src/main.rs` - window, input and the frame loop

## Notes

Physics runs at a fixed 120 steps a second with an accumulator, so the jump is the same height at 30fps or 240fps. After a stall (dragging the window, a breakpoint) the catch-up is capped at 0.25s.

The pipe gaps come from a small xorshift rng seeded from the clock.

The sky is two triangles with a gradient, and the bird is circles and triangles rotated around its centre by how fast it's falling.

## Todo

- [ ] run on real Windows
- [ ] tests for game.rs
- [ ] save the best score
- [ ] renderer as a git dependency instead of a path
