# empetri - music player

## Supported formats:
- flac
- mp3
- ogg

## Supported platforms:
- Linux (native build)
- Windows (from linux to windows)
- Android (from linux to android !UNSTABLE!)

## Build (Linux):
#### Requirements:
- mold

```bash
cargo build --release
```

## Build (from Linux to Windows)
#### Requirements:
```bash
rustup target add x86_64-pc-windows-gnu
```
```bahs
cargo build --release --target=x86_64-pc-windows-gnu
```

## Build (from Linux to Android)
*compile (and build apk with cargo-apk)*
```bash
./buildbymagicomg.sh
```
*instal*
```bash
adb install target/debug/apk/empetri.apk
```

#### LICENSE: GPLv3
