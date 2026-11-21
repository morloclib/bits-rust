// Shift and bit-index semantics:
//   * A negative shift or bit index panics.
//   * shiftl drops bits shifted past the width; a shift >= width gives 0.
//   * shiftr is arithmetic for signed and logical for unsigned types; a shift
//     >= width gives 0, or -1 for a negative signed value.
//   * testBit at an index >= width reads the sign extension.
//   * setBit, clearBit and flipBit panic at an index >= width.

pub trait BitsOps: Copy {
    const WIDTH: i64;
    fn b_shl(self, n: u32) -> Self;
    fn b_shr(self, n: u32) -> Self;
    fn b_fill(self) -> Self;
    fn b_zero() -> Self;
    fn b_not(self) -> Self;
    fn b_bit(n: u32) -> Self;
    fn b_and(self, o: Self) -> Self;
    fn b_or(self, o: Self) -> Self;
    fn b_xor(self, o: Self) -> Self;
    fn b_low(self) -> bool;
    fn b_ones(self) -> i64;
}

macro_rules! impl_bits {
    ($($t:ty),*) => {$(impl BitsOps for $t {
        const WIDTH: i64 = <$t>::BITS as i64;
        fn b_shl(self, n: u32) -> Self { self << n }
        fn b_shr(self, n: u32) -> Self { self >> n }
        #[allow(unused_comparisons)]
        fn b_fill(self) -> Self { if self < 0 { !0 } else { 0 } }
        fn b_zero() -> Self { 0 }
        fn b_not(self) -> Self { !self }
        fn b_bit(n: u32) -> Self { (1 as $t) << n }
        fn b_and(self, o: Self) -> Self { self & o }
        fn b_or(self, o: Self) -> Self { self | o }
        fn b_xor(self, o: Self) -> Self { self ^ o }
        fn b_low(self) -> bool { self & 1 == 1 }
        fn b_ones(self) -> i64 { self.count_ones() as i64 }
    })*};
}
impl_bits!(i8, i16, i32, i64, u8, u16, u32, u64);

fn bits_nonneg(what: &str, n: i64) {
    if n < 0 {
        panic!("{}: negative bit count {}", what, n);
    }
}

fn bits_index<A: BitsOps>(what: &str, n: i64) -> u32 {
    bits_nonneg(what, n);
    if n >= A::WIDTH {
        panic!("{}: bit index {} out of range for a {}-bit integer", what, n, A::WIDTH);
    }
    n as u32
}

pub fn bits_shiftl<A: BitsOps>(x: &A, n: i64) -> A {
    bits_nonneg("shiftl", n);
    if n >= A::WIDTH { A::b_zero() } else { x.b_shl(n as u32) }
}

pub fn bits_shiftr<A: BitsOps>(x: &A, n: i64) -> A {
    bits_nonneg("shiftr", n);
    if n >= A::WIDTH { x.b_fill() } else { x.b_shr(n as u32) }
}

pub fn bits_bnot<A: BitsOps>(x: &A) -> A { x.b_not() }

pub fn bits_testBit<A: BitsOps>(x: &A, n: i64) -> bool {
    bits_nonneg("testBit", n);
    if n >= A::WIDTH { x.b_fill().b_low() } else { x.b_shr(n as u32).b_low() }
}

pub fn bits_setBit<A: BitsOps>(x: &A, n: i64) -> A {
    x.b_or(A::b_bit(bits_index::<A>("setBit", n)))
}

pub fn bits_clearBit<A: BitsOps>(x: &A, n: i64) -> A {
    x.b_and(A::b_bit(bits_index::<A>("clearBit", n)).b_not())
}

pub fn bits_flipBit<A: BitsOps>(x: &A, n: i64) -> A {
    x.b_xor(A::b_bit(bits_index::<A>("flipBit", n)))
}

pub fn bits_popcount<A: BitsOps>(x: &A) -> i64 { x.b_ones() }
