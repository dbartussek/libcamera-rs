# libcamera-rs

Experimental Rust bindings for [libcamera](https://libcamera.org/).

Project structure:
  - [libcamera-sys](./libcamera-sys/) - Low-level unsafe bindings to libcamera. Also contains libcamera [C API shim](./libcamera-sys/c_api/) to make interfacing with C++ code easier.
  - [libcamera-meta](./libcamera-meta/) - Scripts for generating C and Rust code from libcamera controls, properties and formats YAMLs. Mostly used by the [regenerate.sh](./regenerate.sh) script.
  - [libcamera-rs](./libcamera-rs/) - Safe libcamera Rust interface on top of `libcamera-sys`.

## [Documentation](https://lit-robotics.github.io/libcamera-rs/libcamera_rs/index.html)

## Building

`libcamera-sys` requires [libcamera](https://libcamera.org/) installed and accessible via pkg-config. Check official [getting started guide](https://libcamera.org/getting-started.html) on how to build libcamera. Note that libcamera does not have a stable release yet and any API changes might break the build. This also means that any binary distributions (e.g. in Ubuntu 22.04) will likely be too old. This crate is known to build with libcamera rev `a5fdf63`.

No other special dependencies are needed. All crates can be built from the root workspace dir with `cargo build`.

## Running examples

Print `libcamera` version using only `libcamera-sys` ([code](./libcamera-sys/examples/version.rs)):
```console
osboxes@osboxes:~/libcamera-rs$ cargo run --example version
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/examples/version`
libcamera: v0.0.1+50-aa7b3740
```

List cameras with `libcamera-rs` ([code](./libcamera-rs/examples/list_cameras.rs)):
```console
osboxes@osboxes:~/libcamera-rs$ cargo run --example list_cameras
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/list_cameras`
[4:16:17.777208430] [25773]  INFO Camera camera_manager.cpp:293 libcamera v0.0.1+50-aa7b3740
Camera 0
ID: \_SB_.PCI0-2:1.0-0c45:2690
Properties: Immutable(
    {
        PixelArrayActiveAreas: PixelArrayActiveAreas(
            [
                Rectangle {
                    x: 0,
                    y: 0,
                    width: 1920,
                    height: 1080,
                },
            ],
        ),
        PixelArraySize: PixelArraySize(
            Size {
                width: 1920,
                height: 1080,
            },
        ),
        Location: CameraExternal,
        Model: Model(
            "AUKEY PCW1: AUKEY PCW1",
        ),
    },
)
Available formats: {
    MJPEG: [
        Size {
            width: 320,
            height: 240,
        },
...
        Size {
            width: 1920,
            height: 1080,
        },
    ],
    YUYV: [
        Size {
            width: 320,
            height: 240,
        },
...
        Size {
            width: 1920,
            height: 1080,
        },
    ],
}
```

Capture JPEG image into a file ([code](./libcamera-rs/examples/jpeg_capture.rs)):
```console
osboxes@osboxes:~/libcamera-rs$ cargo run --example jpeg_capture target/image.jpg
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/examples/jpeg_capture target/image.jpg`
[4:18:00.104950400] [25950]  INFO Camera camera_manager.cpp:293 libcamera v0.0.1+50-aa7b3740
Using camera: AUKEY PCW1: AUKEY PCW1
Generated config: [
    StreamConfigurationRef {
        pixel_format: MJPEG,
        size: Size {
            width: 1920,
            height: 1080,
        },
        stride: 0,
        frame_size: 4147789,
        buffer_count: 4,
    },
]
[4:18:00.159095868] [25950]  INFO Camera camera.cpp:1026 configuring streams: (0) 1920x1080-MJPEG
Allocated 4 buffers
Waiting for camera request execution
Camera request Request { seq: 0, status: Complete, cookie: 0 } completed!
Metadata: Immutable(
    {
        SensorTimestamp: SensorTimestamp(
            15480581860000,
        ),
    },
)
FrameBuffer metadata: Immutable(
    FrameMetadataRef {
        status: Success,
        sequence: 0,
        timestamp: 15480581860000,
        planes: [
            libcamera_frame_metadata_plane {
                bytes_used: 442672,
            },
        ],
    },
)
Written 4147789 bytes to target/image.jpg
```

## Notes on safety

`libcamera-rs` is intended to be a fully memory-safe wrapper, however, due to `libcamera`'s complexity and many cross-references between objects it is quite hard to ensure total safety so there is very likely to be bugs. Issues and pull requests are welcome.

## FAQ

- Why not wrap C++ API directly instead of using intermediate C layer?
  - Writting a C++ wrapper in Rust is quite difficult because many features do no translate to Rust well: polymorphism, function overloading, templates, etc. There are tools to generate C++ bindings, but they usually break for anything more complex or result in even more boilerplate code than an additional C layer.
- List-like structures (`CameraConfiguration`, `ControlList`) are not indexable
  - It is impossible to implement `Index` and `IndexMut` traits for these structures, because traits can only return reference to an existing data within structure. Most of the libcamera wrappers return newtype variants, making them incompatible with indexing.

# Random notes

- libcamera's `FrameBuffer` has `FrameMetadata`, which by default is uninitialized with no way of checking that. It must only be read once attached `Request` is executed. This is hard to track, because our request accepts trait `AsFrameBuffer` to support other allocators and we can't attach a bool field to a trait. As a temporary fix, we set `FrameMetadata::status` to `u32::MAX` when `FrameBuffer` is created, which we can use to check if metadata was initialized. This way `AsFrameBuffer::metadata()` can return `None` instead of reading uninitialized memory.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
