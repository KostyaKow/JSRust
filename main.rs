//https://gist.github.com/klutzy/7819231
//rustc --emit=llvm-bc hello.rs
//rustc --emit=llvm-ir hello.rs
//https://github.com/rust-lang/rust/tree/master/src/librustc_back/target
//--target=i686_unknown_linux_gnu --target=asmjs_unknown_emscripten
//rustc --target=i686_unknown_linux_gnu --emit=llvm-ir hello.rs
#![feature(intrinsics)]
#![feature(macro_rules)]
#![feature(lang_items)]
#![feature(start)]
#![feature(no_core)]
#![no_std]
#![no_core]

#![feature(inclusive_range_syntax)]
#![feature(intrinsics)]
#![feature(associated_type_defaults)]
#![feature(specialization)]
#![feature(no_core)]
#![feature(fundamental)]
#![feature(reflect)]

#![feature(box_syntax)]
#![feature(on_unimplemented)]

#![feature(allow_internal_unstable)]

//copied from std
#![feature(asm)]
#![feature(associated_consts)]
#![feature(borrow_state)]
#![feature(box_syntax)]
#![feature(cfg_target_vendor)]
#![feature(cfg_target_thread_local)]
#![feature(char_internals)]
#![feature(collections)]
#![feature(collections_bound)]
#![feature(const_fn)]
#![feature(copy_from_slice)]
#![feature(core_float)]
#![feature(core_intrinsics)]
#![feature(decode_utf16)]
#![feature(dropck_parametricity)]
#![feature(float_extras)]
#![feature(float_from_str_radix)]
#![feature(fnbox)]
#![feature(fn_traits)]
#![feature(heap_api)]
#![feature(hashmap_hasher)]
#![feature(inclusive_range)]
#![feature(int_error_internals)]
#![feature(into_cow)]
#![feature(lang_items)]
#![feature(libc)]
#![feature(link_args)]
#![feature(linkage)]
#![feature(macro_reexport)]
#![cfg_attr(test, feature(map_values_mut))]
#![feature(num_bits_bytes)]
#![feature(old_wrapping)]
#![feature(on_unimplemented)]
#![feature(oom)]
#![feature(optin_builtin_traits)]
#![feature(placement_in_syntax)]
#![feature(rand)]
#![feature(raw)]
#![feature(repr_simd)]
#![feature(reflect_marker)]
#![feature(rustc_attrs)]
#![feature(shared)]
#![feature(slice_bytes)]
#![feature(slice_concat_ext)]
#![feature(slice_patterns)]
#![feature(staged_api)]
#![feature(stmt_expr_attributes)]
#![feature(str_char)]
#![feature(str_internals)]
#![feature(str_utf16)]
#![feature(test, rustc_private)]
#![feature(thread_local)]
#![feature(unboxed_closures)]
#![feature(unicode)]
#![feature(unique)]
#![feature(unsafe_no_drop_flag, filling_drop)]
#![feature(unwind_attributes)]
#![feature(vec_push_all)]
#![feature(zero_one)]
#![feature(question_mark)]
//end copied from std

//#[macro_reexport(assert, assert_eq, debug_assert, debu_assert_eq, unreachable, unimplemented, write, writeln, try)]
#[macro_use]
mod macro;
#[path = "num/float_macros.rs"]
#[macro_use]
mod float_macros;

#[path = "num/int_macros.rs"]
#[macro_use]
mod int_macros;

#[path = "num/uint_macros.rs"]
#[macro_use]
mod uint_macros;

mod core;

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

/*extern "rust-intrinsic" {
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
*/

/*use std::ops::Add;

impl Add for u32 {
   type Output = u32;
   fn add(self, _rhs: u32) -> u32 {
      ::add_ints(self, _rhs)
   }
}*/

#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
   //core::io::stdout().write(bytes!("hello world"));

   /*let mut i : u32 = 0;
   let max : u32 = 100;
   while u32_lt(i, max) {
      i = i + 1;
      putchar(u32_to_c(i));
   }

   print_ln("hello world");
   0*/
}

