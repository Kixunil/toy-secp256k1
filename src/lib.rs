use bigint::uint::U256;
use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign};

const P: U256 = U256([0xFFFFFFFE_FFFFFC2F, 0xFFFFFFFF_FFFFFFFF, 0xFFFFFFFF_FFFFFFFF, 0xFFFFFFFF_FFFFFFFF]);

// Convenience methods
trait U256Ext {
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_shl(self, rhs: usize) -> Self;
}

impl U256Ext for U256 {
    fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }

    fn wrapping_sub(self, rhs: Self) -> Self {
        self.overflowing_sub(rhs).0
    }

    fn wrapping_shl(self, rhs: usize) -> Self {
        self << rhs
    }
}

/// Implementation of `Z_p` cyclic group where `p` is the size of the field used in secp256k1 - se
/// the `P` constant in this library.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Zp(U256);

impl Zp {
    pub const ZERO: Self = Zp(U256([0, 0, 0, 0]));

    /// Converts the value % P to Self
    pub fn wrapping_from(value: U256) -> Self {
        if value >= P {
            Zp(value.wrapping_sub(P))
        } else {
            Zp(value)
        }
    }

    pub fn checked_from(value: U256) -> Option<Self> {
        if value >= P {
            None
        } else {
            Some(Zp(value))
        }
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn multiplicative_inverse(self) -> Self {
        Zp(self.0.mod_inverse(P))
    }
}

// We use simple subtraction instead of modulo as it should be more efficient
impl Add for Zp {
    type Output = Self;

    fn add(self, rhs: Zp) -> Self::Output {
        let (res, overflow) = self.0.overflowing_add(rhs.0);
        Zp(if overflow || res >= P {
            res.wrapping_sub(P)
        } else {
            res
        })
    }
}

impl AddAssign for Zp {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Zp {
    type Output = Self;

    fn sub(self, rhs: Zp) -> Self::Output {
        let (res, overflow) = self.0.overflowing_sub(rhs.0);
        Zp(if overflow || res >= P {
            res.wrapping_add(P)
        } else {
            res
        })
    }
}

impl SubAssign for Zp {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<U256> for Zp {
    type Output = Zp;

    /// Double-and-add algorithm
    fn mul(self, mut rhs: U256) -> Self::Output {
        let mut res = Zp::ZERO;

        for _ in 0..256 {
            // Can't use *= 2 - that would cause infinite recursion.
            // Don't ask how I know.
            res += res;
            if rhs & U256([0, 0, 0, 1 << 63]) != U256::zero() {
                res += self;
            }
            rhs = rhs.wrapping_shl(1);
        }

        res
    }
}

impl Mul<u64> for Zp {
    type Output = Zp;

    fn mul(self, rhs: u64) -> Self::Output {
        self * U256::from(rhs)
    }
}

impl MulAssign<u64> for Zp {
    fn mul_assign(&mut self, rhs: u64) {
        *self = *self * rhs;
    }
}

impl Mul for Zp {
    type Output = Zp;

    fn mul(self, rhs: Zp) -> Self::Output {
        self * rhs.0
    }
}

impl MulAssign for Zp {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Zp {
    type Output = Zp;

    fn div(self, rhs: Zp) -> Self::Output {
        self * rhs.multiplicative_inverse()
    }
}

impl DivAssign for Zp {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for Zp {
    type Output = Zp;

    fn neg(self) -> Self::Output {
        if self.is_zero() {
            self
        } else {
            Zp(P - self.0)
        }
    }
}

/// Secp256k1 curve point
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Point {
    x: Zp,
    y: Zp,
}

impl Point {
    /// Point at infinity - neutral element, ironically denoted as 0
    pub const AT_INFINITY: Point = Point { x: Zp::ZERO, y: Zp::ZERO, };

    /// Constructs the point from coordinates.
    ///
    /// Returns `None` if the point is not on the curve
    pub fn new(x: Zp, y: Zp) -> Option<Self> {
        if (x.is_zero() && y.is_zero()) || y * y == x * x * x + B {
            Some(Point { x, y })
        } else {
            None
        }
    }

    /// Checks if the point is neutral element
    pub fn is_at_infinity(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }

    /// Computes multiplicative inverse for scalar multiplication.
    /// 
    /// For each scalar `x`, curve point `P`: `P*x*scalar_multiplicative_inverse(x) == P`.
    /// Or in other words `x*scalar_multiplicative_inverse(x) % curve order == 1`.
    pub fn scalar_multiplicative_inverse(scalar: U256) -> U256 {
        scalar.mod_inverse(SECP256K1_GROUP_ORDER)
    }
}

pub const G: Point = Point { x: Zp(U256([0x59F2815B_16F81798, 0x029BFCDB_2DCE28D9, 0x55A06295_CE870B07, 0x79BE667E_F9DCBBAC])), y: Zp(U256([0x9C47D08F_FB10D4B8, 0xFD17B448_A6855419, 0x5DA4FBFC_0E1108A8, 0x483ADA77_26A3C465])), };
const B: Zp = Zp(U256([7, 0, 0, 0]));

/// Curve order of SECP256K1
const SECP256K1_GROUP_ORDER: U256 = U256([0xBFD25E8C_D0364141, 0xBAAEDCE6_AF48A03B, 0xFFFFFFFF_FFFFFFFE, 0xFFFFFFFF_FFFFFFFF]);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        if self.is_at_infinity() {
            return rhs;
        }
        if rhs.is_at_infinity() {
            return self;
        }
        if self == -rhs {
            return Point::AT_INFINITY;
        }

        // Made it easier to copy from Wikipedia :)
        let q = self;
        let p = rhs;

        let lambda = if p == q {
            // point doubling
            p.x * p.x * 3 /* + a, which is 0 for secp256k1 */ / (p.y * 2)
        } else {
            (q.y - p.y) / (q.x - p.x)
        };

        let x = lambda * lambda - p.x - q.x;
        // Note that there's `x` in the parentheses not `something.x`, this is correct, the font at
        // Wikipedia is awful.
        let y = lambda * (p.x - x) - p.y;

        Point { x, y, }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<U256> for Point {
    type Output = Point;

    // Double and add algorithm - that means **NOT CONSTANT TIME!!!**
    fn mul(self, mut rhs: U256) -> Self::Output {
        let mut res = Point::AT_INFINITY;

        for _ in 0..256 {
            res = res + res;
            if rhs & U256([0, 0, 0, 1 << 63]) != U256::zero() {
                res += self;
            }
            rhs = rhs.wrapping_shl(1);
        }

        res
    }
}

impl Mul<u64> for Point {
    type Output = Point;

    fn mul(self, rhs: u64) -> Self::Output {
        self * U256::from(rhs)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Point, G, Zp};
    use bigint::U256;

    macro_rules! be_point {
        ($xa:expr, $xb:expr, $xc:expr, $xd:expr; $ya:expr, $yb:expr, $yc:expr, $yd:expr) => {
            Point {
                x: Zp(U256([$xd, $xc, $xb, $xa])),
                y: Zp(U256([$yd, $yc, $yb, $ya])),
            }
        }
    }

    const P: Point = be_point!(0x79BE667EF9DCBBAC, 0x55A06295CE870B07, 0x029BFCDB2DCE28D9, 0x59F2815B16F81798; 0x483ADA7726A3C465, 0x5DA4FBFC0E1108A8, 0xFD17B448A6855419, 0x9C47D08FFB10D4B8);

    #[test]
    fn g_is_on_curve() {
        assert_eq!(Point::new(G.x, G.y), Some(G));
    }

    #[test]
    fn p_is_on_curve() {
        assert_eq!(Point::new(P.x, P.y), Some(P));
    }

    #[test]
    fn curve_order() {
        assert!((G * super::SECP256K1_GROUP_ORDER).is_at_infinity());
    }

    #[test]
    fn distributive() {
        assert_eq!(G * (42 + 47), G * 42 + G * 47);
    }

    #[test]
    fn double_is_mul_2() {
        assert_eq!(G * 2, G + G);
    }

    #[test]
    fn triple_is_mul_3() {
        assert_eq!(G * 3, G + G + G);
    }

    #[test]
    fn p_times_2() {
        assert_eq!(P * 2, be_point!(0xC6047F9441ED7D6D, 0x3045406E95C07CD8, 0x5C778E4B8CEF3CA7, 0xABAC09B95C709EE5; 0x1AE168FEA63DC339, 0xA3C58419466CEAEE, 0xF7F632653266D0E1, 0x236431A950CFE52A));
    }

    #[test]
    fn p_times_3() {
        assert_eq!(P * 3, be_point!(0xF9308A019258C310, 0x49344F85F89D5229, 0xB531C845836F99B0, 0x8601F113BCE036F9; 0x388F7B0F632DE814, 0x0FE337E62A37F356, 0x6500A99934C2231B, 0x6CB9FD7584B8E672));
    }

    #[test]
    fn multiplicative_inverse() {
        assert_eq!((G * 42) * Point::scalar_multiplicative_inverse(42.into()), G);
    }
}
