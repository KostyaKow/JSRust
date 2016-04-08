rustc --target=i686_unknown_linux_gnu --emit=llvm-ir hello.rs
emcc hello.ll -o hello.js
