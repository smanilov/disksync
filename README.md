A GUI utility (GTK4-based) to sync file writes and power-off external HDDs on
Linux.

Run with `cargo run`.

Just calls `sync`, `umount`, and `udisksctl power-off`, so make sure you have
those installed.
