A GUI utility (GTK4-based) to sync file writes and power-off external HDDs on
Linux.

Run with `cargo run` (last tested on `cargo 1.86.0 (adf9b6ad1 2025-02-28)`).

Just calls `sync`, `umount`, and `udisksctl power-off`, so make sure you have
those installed.
