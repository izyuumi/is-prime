#[cfg(test)]
mod tests {
    use primality::IsPrime;

    #[test]
    fn unsigned() {
        // u8
        assert!(!0u8.is_prime());
        assert!(2u8.is_prime());
        assert!(!35u8.is_prime());
        assert!(!57u8.is_prime());

        // u16
        assert!(!0u16.is_prime());
        assert!(!1u16.is_prime());
        assert!(2u16.is_prime());
        assert!(3u16.is_prime());
        assert!(!7910u16.is_prime());
        assert!(7919u16.is_prime());

        // u32
        assert!(!0u32.is_prime());
        assert!(13u32.is_prime());
        assert!(!26097u32.is_prime());
        assert!(26099u32.is_prime());

        // u64
        assert!(!4u64.is_prime());
        assert!(5u64.is_prime());
        assert!(!46u64.is_prime());
        assert!(47u64.is_prime());
        assert!(!2256522565u64.is_prime());
        assert!(2256522601u64.is_prime());

        // u128
        assert!(!0u128.is_prime());
        assert!(7u128.is_prime());
        assert!(!1999999999u128.is_prime());
        assert!(2000000011u128.is_prime());

        // usize
        assert!(!4usize.is_prime());
        assert!(5usize.is_prime());
        assert!(!99999usize.is_prime());
        assert!(99991usize.is_prime());
    }

    #[test]
    fn signed() {
        // i8
        let neg_i8: i8 = -1;
        assert!(!neg_i8.is_prime());
        assert!(!0i8.is_prime());
        assert!(!1i8.is_prime());
        assert!(2i8.is_prime());
        assert!(!35i8.is_prime());
        assert!(!57i8.is_prime());

        // i16
        let neg_i16: i16 = -11249;
        assert!(!0i16.is_prime());
        assert!(!1i16.is_prime());
        assert!(2i16.is_prime());
        assert!(3i16.is_prime());
        assert!(!neg_i16.is_prime());
        assert!(!7910i16.is_prime());
        assert!(7919i16.is_prime());

        // i32
        let neg_i32: i32 = -11249;
        assert!(!0i32.is_prime());
        assert!(13i32.is_prime());
        assert!(!neg_i32.is_prime());
        assert!(!26097i32.is_prime());
        assert!(26099i32.is_prime());

        // i64
        let neg_i64: i64 = -32423423;
        assert!(!4i64.is_prime());
        assert!(5i64.is_prime());
        assert!(!46i64.is_prime());
        assert!(47i64.is_prime());
        assert!(!neg_i64.is_prime());
        assert!(!2256522565i64.is_prime());
        assert!(2256522601i64.is_prime());

        // i128
        let neg_i128: i128 = -32423423123423;
        assert!(!0i128.is_prime());
        assert!(7i128.is_prime());
        assert!(!neg_i128.is_prime());
        assert!(!1999999999i128.is_prime());
        assert!(2000000011i128.is_prime());

        // isize
        let neg_isize: isize = -12341245;
        assert!(!4isize.is_prime());
        assert!(5isize.is_prime());
        assert!(!99999isize.is_prime());
        assert!(99991isize.is_prime());
        assert!(!neg_isize.is_prime());
    }
}
