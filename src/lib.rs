use num_integer::Integer;

pub trait IsPrime: Integer + Clone {
    fn is_prime(&self) -> bool {
        if self.is_zero() || self.is_one() || self.le(&Self::zero()) {
            return false;
        }
        let rem_div_two = self.div_rem(&(Self::one() + Self::one()));
        if rem_div_two.0.is_one() && rem_div_two.1.is_zero() {
            return true;
        }
        if self.is_even() {
            return false;
        }

        let mut cur = Self::one() + Self::one() + Self::one();
        while cur.clone().mul(cur.clone()).le(self) {
            if self.div_rem(&cur).1.is_zero() {
                return false;
            }
            cur = cur + Self::one() + Self::one();
        }

        true
    }
}

impl IsPrime for u8 {}
impl IsPrime for u16 {}
impl IsPrime for u32 {}
impl IsPrime for u64 {}
impl IsPrime for u128 {}
impl IsPrime for usize {}
impl IsPrime for i8 {}
impl IsPrime for i16 {}
impl IsPrime for i32 {}
impl IsPrime for i64 {}
impl IsPrime for i128 {}
impl IsPrime for isize {}
