
default: test

test:
	make clean; make js; node test.js.test

#emcc rust_std.c core.ll main.ll -o hello.js
js: main
	emcc rust_std.c main.ll -o hello.js

#rustc --crate-name list --crate-type lib list.rs
#rustc --crate-name utils --crate-type lib utils.rs
#rustc -L . lisp.rs

main:
	rustc --target=i686_unknown_linux_gnu --emit=llvm-ir main.rs

clean:
	rm -f *.ll *.js
