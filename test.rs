// emscripten uses le32-unknown-nacl triple but rustc doesn't know it now.
// So just use similar target instead.
// `rustc hello.rs --target=i686-unknown-linux --emit-llvm -S --cfg libc`
// `emcc hello.ll -o hello.js`
//rustc --emit=llvm-bc hello.rs
//rustc --emit=llvm-ir hello.rs
//--target=i686_unknown_linux_gnu



#![feature(lang_items)]
#![feature(start)]
#![no_std]

#![feature(no_core)]

//#![feature(lang_items)]
//#[feature(macro_rules)]
#![no_core]
#![feature(lang_items)]
//use core::container::Container;


#[link(name = "test")]
extern {
   fn test(input: *const u8) -> u32;
}

#[lang="sized"]
trait Sized {}


#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
   let mut i : u32 = 0;
   while (i < 1000000) {
      i=i+1;
      let mut j : u32 = 0;
      while (j < 1000000) {
         j=j+1;
         let mut z : u32 = 0;
         while z < 1000000 {
            z=z+1;
         }
      }
   }
}


