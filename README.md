[![Build Status](https://travis-ci.org/rasendubi/bkernel.svg)](https://travis-ci.org/rasendubi/bkernel)

bkernel is an experimental kernel for embedded devices written in Rust. I'm mostly trying out Rust now to see how it applies to kernel development.

# Prerequisites

## gcc-arm-none-eabi toolchain

You need an gcc-arm-none-eabi toolchain before you can build the kernel.

If you don't know where to get one, you can get it [there](https://launchpad.net/gcc-arm-embedded/+download):

- Download one for your platform
- Unpack
- Add `<path_to_toolchain>/bin` to your `$PATH` variable

## Rust version

This project needs lots of nightly features:

- inline assembly
- no stdlib
- intrinsics
- lang items

Nightly builds are not backward-compatible, so only the latest version is supported (it changes every 6 weeks). That's why you need a reasonably up-to-date nightly rust.

## Rust sources

bkernel needs Rust sources to build libcore for the target. If you don't have one, don't worry: it will be automatically downloaded to `rust` directory.

If you have Rust git repo on you computer, you can point to it with:

```sh
export RUSTDIR=/path/to/rust
```

Note: building bkernel will checkout rust to a commit your rustc was compiled with.

# Build instructions

Just invoke `make`.

## Make targets

- `make` build all binaries, documentation and run tests;
- `make build` only build kernel;
- `make test` run tests;
- `make doc` build documentation.

# Flashing

If you have a STM32F4Discovery board, you can flash it in the following way:

- Run openocd with `openocd -f openocd.cfg`.
- Connect to the running server with `telnet localhost 4444` and run `flash_bkernel kernel.bin` command.

# Running

After booting the kernel you should see all LEDs are turned on and a terminal is running on PB6/PB7 pins.

The following commands are supported:
- `hi` - says hello
- `+3`/`-3`/`+4`/`-4`/`+5`/`-5`/`+6`/`-6` - turn on/off LD3/4/5/6
- `panic` - throw a panic

# Issues

If you have any issues or questions with the bkernel, just [open an issue](https://github.com/rasendubi/bkernel/issues) or mail me at [rasen.dubi@gmail.com](mailto:rasen.dubi@gmail.com).

# License

The bkernel source code is licensed by a modified GNU General Public License - the modification taking a form of an exception. The exception permits the source code of applications that use bkernel and are distributed as executables to remain closed source, thus permitting the use of bkernel in commercial applications without necessitating that the whole application be open sourced. The exception can only be used if you wish to combine bkernel with a proprietary product, and you comply with the terms stated in the exception itself.

The full text of the bkernel license is available [here](LICENSE).
