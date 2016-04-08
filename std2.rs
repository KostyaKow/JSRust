
pub mod std {
   pub mod ops {
      pub trait Add<RHS = Self> {
         type Output;

         fn add(self, rhs: RHS) -> Self::Output;
      }
      pub impl Add for u32 {
         type Output = u32;
         fn add(self, _rhs: u32) -> u32 {
            ::add_ints(self, _rhs)
         }
      }
   }
}
