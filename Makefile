
default: test

test:
	make clean; make stdtest; node test.js.test

stdtest: stdrustlib
	emcc rust_std.c std.ll -o hello.js

stdrustlib:
	rustc --target=i686_unknown_linux_gnu --emit=llvm-ir std.rs

clean:
	rm -f *.ll *.js
