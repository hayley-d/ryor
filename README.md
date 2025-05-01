# ryor
A minimal rust OS

### How to Compile
```bash
# Linux
cargo +nightly build --target target.json
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```


### How to Create Bootable Disk Image
```bash
cargo bootimage
```
