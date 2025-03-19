# Reproducer for Zed issue #26143

See https://github.com/zed-industries/zed/issues/26143.

1. `cargo run`

   (if the app window crashes immediately do `cargo run 500` and find the crashing length either by running
   again with a higher number or pressing the Increase button)
2. Increase the window width if the app tells you to.
3. Press *Increase* until the window crashes (for me usually it happens for 1025 or 1026).
