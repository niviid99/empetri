# empetri - music player

## Supported formats:
- flac
- mp3
- ogg

## Supported platforms:
- Linux (native build)
- Windows (from linux to windows)
- Android (*in android branch*)

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


#### LICENSE: GPLv3
