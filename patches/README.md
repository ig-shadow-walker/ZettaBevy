# Patched dependencies

Copies of libraries Bevy depends on, with fixes Z3D games need that the libraries
haven't released yet. Bevy's own builds use them through `[patch.crates-io]` in the root
`Cargo.toml`, and `z3d` points every game at them the same way.

Each one is temporary: when the library releases the fix and Bevy moves to that
version, delete the copy and its `[patch.crates-io]` line.

## winit 0.30.13

winit is the library Bevy uses to open the game's window and receive input.

**The problem:** iOS 27 only starts apps built with its SDK if they use the UIScene life
cycle. winit 0.30 doesn't, so every Bevy game closes as soon as it starts on iOS 27.

**The fix:** an app declares a scene in its Info.plist (`z3d build --target ios` writes
it), and winit attaches its window to the scene iOS creates, since iOS only shows windows
that belong to a scene. Without the second part the game runs behind a blank screen.

**What changed** compared with winit 0.30.13 from crates.io:

- `Cargo.toml`: the `UIScene` and `UIWindowScene` features of `objc2-ui-kit`.
- `src/platform_impl/ios/app_state.rs`: scenes exist from iOS 13 (`scenes: 13-0`), so
  older iOS versions skip the new step.
- `src/platform_impl/ios/window.rs`: after creating a window, attach it to the connected
  window scene if it has none.

It's the change proposed to winit in
[rust-windowing/winit#4723](https://github.com/rust-windowing/winit/pull/4723), as
backported to 0.30.13. The rest of the folder is winit 0.30.13 as published.

**Remove it when** Bevy depends on a winit release with UIScene support.
