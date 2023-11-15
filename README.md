# Gearbox Widget

An application to emulate a gearbox sending messages to the
Vehicle HAL of a guest Android Auomotive OS through a socket.

<div align="center">
    <img src="./data/screenshot.png" />
</div>

In order to connect to the Vehicle HAL daemon on the guest, the
widget uses the `vhal_emulator`, which in turn, it relies on
`adb` to do a port forwarding before attaching the socket.
Thefore, the widget needs to be able to find `adb`.

# Running the application

1 - Install Rust using rustup <https://rustup.rs/>

2 - Make sure adb is in the `$PATH` or that the `$ADB_PATH` variable is set

2 - Run the application

```shell
ADB_PATH=/path/to/adb/bin cargo run
```
