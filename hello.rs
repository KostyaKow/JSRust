// emscripten uses le32-unknown-nacl triple but rustc doesn't know it now.
// So just use similar target instead.
// `rustc hello.rs --target=i686-unknown-linux --emit-llvm -S --cfg libc`
// `emcc hello.ll -o hello.js`
//https://gist.github.com/klutzy/7819231
//rustc --emit=llvm-bc hello.rs
//rustc --emit=llvm-ir hello.rs
//https://github.com/rust-lang/rust/tree/master/src/librustc_back/target

//--target=i686_unknown_linux_gnu --target=asmjs_unknown_emscripten
//rustc --target=i686_unknown_linux_gnu --emit=llvm-ir hello.rs

#![feature(macro_rules)]
#![feature(lang_items)]
#![feature(start)]

#![feature(no_core)]
#![no_std]
#![no_core]


//use core::container::Container;
#[link(name = "test")]
extern {
   //fn test(input: *const u8) -> u32;
   fn test5(input: char) -> isize;

}

#[lang="sized"]
trait Sized {}

#[lang="copy"]
trait Copy {}

fn test() -> isize {
   unsafe { test5('z'); }
   5
}

#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
   /*let mut i : u32 = 0;
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
   }*/
   test()
}


