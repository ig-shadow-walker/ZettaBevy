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

## wgpu-hal 30.0.2

wgpu-hal is the part of wgpu (Bevy's GPU library) that talks to each graphics API; this
patch only changes its Metal backend (Mac, iPhone, iPad).

**The problem:** wgpu opens a new Metal command buffer for every render or compute pass,
plus one for the barriers before it and one for each submitted command buffer's resource
transitions. On Metal, barriers and transitions do nothing, so most of those command
buffers are empty, but each still costs CPU and GPU time, and Metal allows at most 4,096
waiting at once. Bevy draws each shadow view (six per point light) with about five passes
of its own, so a scene with a few dozen shadowed point lights went past the limit, which
wgpu treats as a lost device: the game quit. Below the limit, frames were slow: 8
shadowed point lights took 22 ms per frame on an M4 Pro.

**The fix:** a Metal command buffer is created when something is first recorded into
it, not when encoding starts; encodings that record nothing have none, and submitting
skips them. 8 shadowed point lights now take 8 ms per frame, and 72 shadowed lights run
(they crashed before). Every scene in `Z3D-Engine/benchmarks/big-scenes` got faster;
the city scene's frames took half as long.

**What changed** compared with wgpu-hal 30.0.2 from crates.io, all in `src/metal/`:

- `mod.rs`: `CommandEncoder` keeps the label for a command buffer it hasn't created yet
  (`pending_label`) and the debug groups pushed before it existed (`skipped_debug_groups`);
  `CommandBuffer::raw` is an `Option` (none when nothing was recorded); `Queue::submit`
  commits only real command buffers and signals the fence on the last real one.
- `command.rs`: `begin_encoding` only remembers the label (or creates the command buffer
  at once when strict event sync is on, whose wait value has to be read then);
  `cmd_buf()` creates it on first use, with the 4,096 limit check `begin_encoding` had;
  every place that used the command buffer goes through `cmd_buf()`; residency sets and
  dropping skip encodings that recorded nothing.
- `device.rs`: the new fields start empty.

The rest of the folder is wgpu-hal 30.0.2 as published.

**Remove it when** wgpu stops opening a Metal command buffer per pass (or creates them
lazily itself) and Bevy moves to that version.
