to test, install emscripten and run make

based on this:
https://gist.github.com/klutzy/7819231

rustc --target=i686_unknown_linux_gnu --emit=llvm-ir main.rs

rustc --target=i686_unknown_linux_gnu --emit=llvm-ir hello.rs
emcc hello.ll -o hello.js



compile core library:
/root/fun/rust/emscript/rust/src/libcore
rustc --target=i686_unknown_linux_gnu --emit=llvm-ir lib.rs
