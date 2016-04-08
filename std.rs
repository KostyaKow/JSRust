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
#![feature(intrinsics)]


#[link(name = "test")]
extern {
   //fn test(input: *const u8) -> u32;
   fn c_putchar(input: char);
   fn c_add_ints(a : u32, b : u32) -> u32;
   fn c_int_to_char(a : u32) -> char;
   fn c_int_lt(a : u32, b : u32) -> bool;
   fn c_int_gt(a : u32, b : u32) -> bool;
   fn c_int_eq(a : u32, b : u32) -> bool;
   //fn c_printf(s : *const u8);
   fn c_print_ln(s : *const u8);
   fn c_panic(s : *const u8);
   //fn c_to_ptr(a : u64) ->
}

extern "rust-intrinsic" {
   fn transmute<T, U>(x: T) -> U;
   fn offset<T>(dst: *const T, offset: isize) -> *const T;
}

#[lang="sized"]
trait Sized {}

#[lang="copy"]
trait Copy {}

#[lang="panic"]
pub fn panic(expr_file_line: &(&'static str, &'static str, u32)) {
   //unsafe { c_panic(transmute::<&&str, *const u8>(&expr_file_line) as *const u8); }
}

#[lang="add"]
trait Add<RHS = Self> {
   type Output;
   fn add(self, rhs: RHS) -> Self::Output;
}
impl Add for u32 {
   type Output = u32;
   fn add(self, _rhs: u32) -> u32 {
      u32_add(self, _rhs)
   }
}


fn print_ln(src: &str) {
   unsafe { c_print_ln(transmute::<&&str, *const u8>(&src) as *const u8); }
}
fn putchar(c : char)  {
   unsafe { c_putchar(c); }
}
fn u32_add(a : u32, b : u32) -> u32 {
   unsafe { c_add_ints(a, b) }
}
fn u32_to_c(n : u32) -> char {
   unsafe { c_int_to_char(n) }
}

fn u32_lt(a : u32, b : u32) -> bool {
   unsafe { c_int_lt(a, b) }
}


/*use std::ops::Add;

impl Add for u32 {
   type Output = u32;
   fn add(self, _rhs: u32) -> u32 {
      ::add_ints(self, _rhs)
   }
}*/

#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
   let mut i : u32 = 0;
   let max : u32 = 100;
   while u32_lt(i, max) {
      i = i + 1;
      putchar(u32_to_c(i));
   }

   print_ln("hello world");
   0
}

