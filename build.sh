#!/bin/bash
#cargo build --lib --target x86_64-pc-windows-gnu --release
# For sliver extensions
#cp /home/zdroid/Documents/Pentesting/sliver-oxide/target/x86_64-pc-windows-gnu/release/sliver_oxide.dll .
# For Sliver Bof Development
#x86_64-w64-mingw32-objcopy -O pe-x86-64 sliver_oxide.dll sliver_oxide.o


#rustc --target x86_64-pc-windows-gnu -C opt-level=z -C panic=abort -C link-args=-Wl,--subsystem,windows --crate-type=staticlib src/lib.rs -o sliver_oxide.o




RUSTFLAGS="-C target-cpu=x86-64 -C target-feature=+crt-static -C link-arg=-nostartfiles -C link-arg=-nodefaultlibs -C link-arg=-Wl,--gc-sections" \
rustc --target x86_64-pc-windows-gnu \
    -C opt-level=z \
    -C panic=abort \
    -C debuginfo=0 \
    -C strip=symbols \
    -C codegen-units=1 \
    -C embed-bitcode=no \
    --emit=obj \
    src/lib.rs -o objects/rust_part.o

x86_64-w64-mingw32-gcc -c src/entry.c -o objects/c_part.o

x86_64-w64-mingw32-ld -r objects/rust_part.o objects/c_part.o -o objects/combined.o


x86_64-w64-mingw32-objcopy \
    --remove-section=.drectve \
    --strip-symbol=@feat.00 \
    --remove-section=.data \
    --remove-section=.bss \
    --strip-symbol=rust_begin_unwind \
    --strip-debug \
    objects/combined.o \
    bof_oxide.o

