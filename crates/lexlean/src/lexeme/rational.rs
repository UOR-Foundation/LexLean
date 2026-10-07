//! Exact rational arithmetic for the §34.2 contractivity receipt.
//!
//! A receipt carries `‖A·diag(λ)‖₁` as a reduced `[numerator, denominator]`
//! pair, and it carries it exactly because a float would make the receipt
//! unreproducible on the platforms §10 requires artifacts to agree on. Two
//! machines that disagree in the last bit of an `f64` would publish different
//! receipts for the same lexeme, and a transparency log whose entries depend on
//! the hardware that computed them is not a log.
//!
//! Every value is kept normalized: the fraction is reduced by its greatest
//! common divisor and the denominator is made positive, so `1/2` has exactly
//! one representation. That is what lets a receipt be *compared* rather than
//! merely inspected --- two receipts agree exactly when their pairs are equal,
//! with no cross-multiplication and no tolerance.
//!
//! The type is deliberately small. It has no division, no negative powers, and
//! no infinity, because §34.2 needs column sums and one comparison, and a
//! numeric tower that cannot represent a value this section requires would
//! push the obligation onto a caller --- which is how a float gets into a
//! receipt in the first place.

use std::cmp::Ordering;
use std::fmt;

/// An exact rational in lowest terms with a positive denominator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rational {
    numerator: i128,
    denominator: i128,
}

impl Rational {
    /// Zero, in lowest terms.
    pub const ZERO: Self = Self {
        numerator: 0,
        denominator: 1,
    };

    /// One, in lowest terms.
    pub const ONE: Self = Self {
        numerator: 1,
        denominator: 1,
    };

    /// The rational `numerator / denominator`, reduced.
    ///
    /// # Errors
    /// Returns the reason when `denominator` is zero. Every other input is
    /// accepted and normalized, so `Rational::new(2, 4)` is `1/2` and
    /// `Rational::new(1, -2)` is `-1/2`.
    pub fn new(numerator: i128, denominator: i128) -> Result<Self, String> {
        if denominator == 0 {
            return Err("a rational with a zero denominator has no value".to_owned());
        }
        let (mut numerator, mut denominator) = (numerator, denominator);
        if denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
        }
        let divisor = gcd(numerator.unsigned_abs(), denominator.unsigned_abs());
        if divisor > 1 {
            numerator /= divisor as i128;
            denominator /= divisor as i128;
        }
        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// The rational `numerator / 1`.
    ///
    /// # Errors
    /// Never; the signature matches [`Self::new`] so a caller can lift an
    /// integer into the same expression without a second path.
    pub fn from_integer(numerator: i128) -> Result<Self, String> {
        Self::new(numerator, 1)
    }

    /// This value in lowest terms, as the receipt's `[numerator, denominator]`
    /// pair.
    ///
    /// The pair is what §34.2 puts in the receipt, so a receipt is
    /// self-describing: a verifier reads the two integers and needs no
    /// agreement with this crate about how to reduce them.
    #[must_use]
    pub fn to_pair(self) -> (i128, i128) {
        (self.numerator, self.denominator)
    }

    /// The reduced pair as JSON integers, for a receipt's canonical JSON.
    #[must_use]
    pub fn to_json(self) -> crate::artifact::canonical_json::Json {
        crate::artifact::canonical_json::Json::Arr(vec![
            crate::artifact::canonical_json::Json::Int(self.numerator as i64),
            crate::artifact::canonical_json::Json::Int(self.denominator as i64),
        ])
    }

    /// Read a `[numerator, denominator]` pair back, refusing a pair that is not
    /// already in lowest terms.
    ///
    /// # Errors
    /// Returns the reason when the value is not a two-element array of
    /// integers, when the denominator is zero or negative, or when the pair is
    /// not reduced. The last is what makes the pair a *canonical* encoding:
    /// accepting `2/4` would let two byte-different receipts denote one norm,
    /// and §33.2's leaf hash is over the entry's exact bytes.
    pub fn from_pair(value: &crate::artifact::canonical_json::Json) -> Result<Self, String> {
        let crate::artifact::canonical_json::Json::Arr(items) = value else {
            return Err("an exact rational is a two-element array".to_owned());
        };
        if items.len() != 2 {
            return Err(format!(
                "an exact rational has two elements, found {}",
                items.len()
            ));
        }
        let component = |index: usize| -> Result<i128, String> {
            match items.get(index) {
                Some(crate::artifact::canonical_json::Json::Int(number)) => Ok(i128::from(*number)),
                _ => Err("a rational component is not an integer".to_owned()),
            }
        };
        let numerator = component(0)?;
        let denominator = component(1)?;
        let rational = Self::new(numerator, denominator)?;
        if rational.to_pair() != (numerator, denominator) {
            return Err(format!(
                "the rational {numerator}/{denominator} is not in lowest terms; the reduced form is {}/{}",
                rational.numerator, rational.denominator
            ));
        }
        Ok(rational)
    }

    /// The sum of two rationals.
    ///
    /// # Errors
    /// Returns the reason when the cross-multiplication would overflow `i128`.
    /// Overflow is reported rather than wrapped or saturated: a receipt that
    /// quietly carried a wrong norm would be worse than one that refused.
    ///
    /// `std::ops::Add` is deliberately not implemented: its contract has no way
    /// to report an overflow, so the arithmetic would have to wrap silently, and
    /// a silently wrapped norm is the failure the exactness claim exists to
    /// exclude.
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Self) -> Result<Self, String> {
        let left = self
            .numerator
            .checked_mul(other.denominator)
            .ok_or_else(|| "the rational sum overflowed".to_owned())?;
        let right = other
            .numerator
            .checked_mul(self.denominator)
            .ok_or_else(|| "the rational sum overflowed".to_owned())?;
        let numerator = left
            .checked_add(right)
            .ok_or_else(|| "the rational sum overflowed".to_owned())?;
        let denominator = self
            .denominator
            .checked_mul(other.denominator)
            .ok_or_else(|| "the rational sum overflowed".to_owned())?;
        Self::new(numerator, denominator)
    }

    /// Whether this value is strictly below one.
    #[must_use]
    pub fn is_below_one(self) -> bool {
        self < Self::ONE
    }

    /// This value raised to a non-negative integer power, by repeated
    /// squaring.
    ///
    /// This is how §34.3's `2^(-index)` is held: the reciprocal of
    /// `2^index`, so the result is a reduced fraction with a power-of-two
    /// denominator and never a float.
    ///
    /// # Errors
    /// Returns the reason when a multiplication would overflow `i128`.
    pub fn pow_u128(self, exponent: u32) -> Result<Self, String> {
        let mut result = Self::ONE;
        let mut base = self;
        let mut remaining = exponent;
        while remaining > 0 {
            if remaining & 1 == 1 {
                result = result.mul(base)?;
            }
            remaining >>= 1;
            if remaining > 0 {
                base = base.mul(base)?;
            }
        }
        Ok(result)
    }

    /// The reciprocal of this value.
    ///
    /// # Errors
    /// Returns the reason when this value is zero.
    pub fn recip(self) -> Result<Self, String> {
        if self.numerator == 0 {
            return Err("zero has no reciprocal".to_owned());
        }
        Self::new(self.denominator, self.numerator)
    }

    /// The product of two rationals.
    ///
    /// # Errors
    /// Returns the reason when a multiplication would overflow `i128`.
    // As with `add`: `std::ops::Mul` has no failure channel, so implementing it
    // would make overflow silent.
    #[allow(clippy::should_implement_trait)]
    pub fn mul(self, other: Self) -> Result<Self, String> {
        let numerator = self
            .numerator
            .checked_mul(other.numerator)
            .ok_or_else(|| "the rational product overflowed".to_owned())?;
        let denominator = self
            .denominator
            .checked_mul(other.denominator)
            .ok_or_else(|| "the rational product overflowed".to_owned())?;
        Self::new(numerator, denominator)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        // Denominators are positive by construction, so only the sign of the
        // numerators has to be handled before the magnitudes are compared.
        match (self.numerator.signum(), other.numerator.signum()) {
            (left, right) if left != right => return left.cmp(&right),
            (0, 0) => return Ordering::Equal,
            _ => {}
        }
        let negative = self.numerator < 0;
        let forward = compare_magnitudes(
            self.numerator.unsigned_abs(),
            self.denominator.unsigned_abs(),
            other.numerator.unsigned_abs(),
            other.denominator.unsigned_abs(),
        );
        if negative {
            forward.reverse()
        } else {
            forward
        }
    }
}

/// Compare `left / left_denominator` with `right / right_denominator`, where
/// both numerators are non-negative and both denominators are positive.
///
/// The direct route, cross-multiplying, computes
/// `left · right_denominator - right · left_denominator`, which overflows `i128`
/// as soon as both denominators are large --- and §34.3's `κ` reaches a
/// denominator of `2^126`, so that case is reachable, not hypothetical. A
/// fallback that ordered the pairs as strings would return a *numerically wrong*
/// answer for exactly those inputs, so there is no string fallback here at
/// all. Instead the magnitudes are compared by continued fractions: peel the
/// integer part off each, and when the integer parts agree invert the remainders
/// and continue. Inversion reverses the order, which the `flipped` flag undoes
/// on the way out. The Euclidean algorithm terminates, so the loop ends.
///
/// All four values fit in `u128` because a numerator's magnitude is at most
/// `2^127` and a positive denominator is at most `i128::MAX`, so no step of this
/// comparison can overflow either.
fn compare_magnitudes(
    mut left: u128,
    mut left_denominator: u128,
    mut right: u128,
    mut right_denominator: u128,
) -> Ordering {
    debug_assert!(left_denominator > 0 && right_denominator > 0);
    let mut flipped = false;
    loop {
        let left_quotient = left / left_denominator;
        let right_quotient = right / right_denominator;
        if left_quotient != right_quotient {
            return finish(left_quotient.cmp(&right_quotient), flipped);
        }
        let left_remainder = left % left_denominator;
        let right_remainder = right % right_denominator;
        match (left_remainder, right_remainder) {
            (0, 0) => return finish(Ordering::Equal, flipped),
            (0, _) => return finish(Ordering::Less, flipped),
            (_, 0) => return finish(Ordering::Greater, flipped),
            _ => {}
        }
        // left  = q + left_remainder / left_denominator = q + 1 / (left_denominator / left_remainder)
        // right = q + 1 / (right_denominator / right_remainder)
        // Comparing those reciprocals reverses the order.
        left = left_denominator;
        left_denominator = left_remainder;
        right = right_denominator;
        right_denominator = right_remainder;
        flipped = !flipped;
    }
}

/// Undo the parity of the inversions [`compare_magnitudes`] performed.
fn finish(ordering: Ordering, flipped: bool) -> Ordering {
    if flipped {
        ordering.reverse()
    } else {
        ordering
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.numerator, self.denominator)
    }
}

/// The greatest common divisor of two non-negative values, by Euclid.
fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

#[cfg(test)]
mod tests {
    use super::Rational;
    use std::cmp::Ordering;

    #[test]
    fn every_value_is_reduced_with_a_positive_denominator() {
        assert_eq!(Rational::new(2, 4).expect("reduces").to_pair(), (1, 2));
        assert_eq!(Rational::new(-2, 4).expect("reduces").to_pair(), (-1, 2));
        assert_eq!(Rational::new(1, -2).expect("flips").to_pair(), (-1, 2));
        assert_eq!(Rational::new(0, 5).expect("zero").to_pair(), (0, 1));
        assert_eq!(Rational::new(7, 7).expect("reduces").to_pair(), (1, 1));
    }

    #[test]
    fn ordering_agrees_with_cross_multiplication_where_that_does_not_overflow() {
        // Reference oracle: with small operands the direct cross-multiplication
        // is exact, so the continued-fraction comparison must agree with it.
        let mut checked = 0usize;
        for left in -12i128..=12 {
            for left_denominator in 1..=12i128 {
                for right in -12i128..=12 {
                    for right_denominator in 1..=12i128 {
                        let a = Rational::new(left, left_denominator).expect("valid");
                        let b = Rational::new(right, right_denominator).expect("valid");
                        let left_side = a.numerator * b.denominator;
                        let right_side = b.numerator * a.denominator;
                        assert_eq!(a.cmp(&b), left_side.cmp(&right_side), "{a} against {b}");
                        assert_eq!(a < b, left_side < right_side, "{a} < {b}");
                        checked += 1;
                    }
                }
            }
        }
        assert_eq!(
            checked,
            25 * 12 * 25 * 12,
            "the oracle must cover the whole grid"
        );
    }

    #[test]
    fn ordering_stays_exact_where_cross_multiplication_overflows() {
        // §34.3's κ reaches a denominator of 2^126, so a real lexeme produces
        // rationals a direct cross-multiplication cannot handle. The old
        // fallback ordered the pairs as strings, which called 1/2^126 *larger*
        // than 1/2^125 because "1/4253..." sorts above "1/8507...". These are
        // the two failure cases.
        // A larger denominator is a smaller value: 1/2^125 > 1/2^126.
        let halved_once = Rational::new(1, 1i128 << 125).expect("valid");
        let halved_twice = Rational::new(1, 1i128 << 126).expect("valid");
        assert!(
            halved_twice < halved_once,
            "1/2^126 must compare below 1/2^125"
        );
        assert!(halved_once > halved_twice);
        assert_ne!(halved_once.cmp(&halved_twice), Ordering::Equal);

        // And a pair where cross-multiplication genuinely overflows: a large
        // numerator against a large denominator.
        let left = Rational::new(i64::MAX as i128, 1i128 << 126).expect("valid");
        let right = Rational::new(1, 1i128 << 125).expect("valid");
        assert_eq!(
            left.numerator.checked_mul(right.denominator),
            None,
            "the overflow must be real, or this case is not testing the fallback"
        );
        // left = (2^63 - 1) / 2^126 and right = 2 / 2^126, so left is far above
        // right even though the only way to see that is the comparison itself.
        assert!(
            left > right,
            "the continued fraction must still decide exactly"
        );
        assert!(right < left);

        // The string fallback, had it been reached, would have compared these
        // two by their spelling: "1/4253..." against "1/8507...", which sorts
        // the shorter denominator first and so reports the *opposite* of the
        // truth this test asserts above.
        assert_eq!(
            halved_once.to_string().cmp(&halved_twice.to_string()),
            Ordering::Less,
            "the string order is what the old fallback would have returned"
        );
        assert_eq!(
            halved_once.cmp(&halved_twice),
            Ordering::Greater,
            "and it is the wrong answer, which is why there is no string fallback"
        );
    }

    #[test]
    fn ordering_is_a_total_order_over_signed_values() {
        let values = [
            Rational::new(-3, 2).expect("valid"),
            Rational::new(-1, 1).expect("valid"),
            Rational::new(0, 1).expect("valid"),
            Rational::new(1, 1).expect("valid"),
            Rational::new(3, 2).expect("valid"),
            Rational::new(2, 1).expect("valid"),
        ];
        for left in values {
            for right in values {
                assert_eq!(
                    left.cmp(&right),
                    right.cmp(&left).reverse(),
                    "antisymmetry: {left} against {right}"
                );
                if left.cmp(&right) == Ordering::Equal {
                    assert_eq!(left, right, "equality must be value equality");
                }
            }
        }
        assert!(values[0] < values[2], "a negative is below zero");
        assert!(values[2] < values[5], "zero is below a positive");
    }

    #[test]
    fn a_zero_denominator_has_no_value() {
        assert!(Rational::new(1, 0).is_err());
    }

    #[test]
    fn an_unreduced_pair_is_refused_on_the_way_back() {
        // The pair in a receipt is a canonical encoding, so `2/4` and `1/2`
        // must not both be accepted: they are different bytes and §33.2's leaf
        // hash is over bytes.
        let json = crate::artifact::canonical_json::Json::Arr(vec![
            crate::artifact::canonical_json::Json::Int(2),
            crate::artifact::canonical_json::Json::Int(4),
        ]);
        let error = Rational::from_pair(&json).expect_err("2/4 is not reduced");
        assert!(error.contains("not in lowest terms"), "{error}");
    }

    #[test]
    fn addition_and_comparison_are_exact() {
        // The four probes run against the PIRTM compiler itself returned
        // exactly these values, so they are the cases that must agree.
        let four_fifths = Rational::new(4, 5).expect("valid");
        let one_fifth = Rational::new(1, 5).expect("valid");
        assert_eq!(four_fifths.add(one_fifth).expect("adds").to_pair(), (1, 1));
        assert!(Rational::new(1, 2).expect("valid").is_below_one());
        assert!(!Rational::new(1, 1).expect("valid").is_below_one());
        assert!(Rational::new(4, 5).expect("valid") < Rational::ONE);
        assert!(Rational::new(0, 1).expect("valid") < Rational::new(1, 3).expect("valid"));
    }

    #[test]
    fn repeated_squaring_matches_a_direct_product() {
        let two = Rational::from_integer(2).expect("valid");
        let by_power = two.pow_u128(10).expect("2^10");
        assert_eq!(by_power.to_pair(), (1024, 1));
        let half = Rational::new(1, 2).expect("valid");
        assert_eq!(half.pow_u128(4).expect("valid").to_pair(), (1, 16));
    }

    #[test]
    fn overflow_is_reported_rather_than_wrapped() {
        let huge = Rational::new(i128::MAX, i128::MAX - 1).expect("valid");
        assert!(huge.mul(huge).is_err(), "a product past i128 must refuse");
    }
}

#[cfg(kani)]
mod kani_tests {
    use super::*;

    #[kani::proof]
    fn kani_rational_reduction_maintains_positive_denominator() {
        let n = kani::any::<i128>();
        let d = kani::any::<i128>();
        kani::assume(d != 0);

        if let Ok(r) = Rational::new(n, d) {
            let (rn, rd) = r.to_pair();
            assert!(rd > 0, "denominator must be positive");

            // Should also not panic and correctly handle signs
            if n > 0 && d > 0 {
                assert!(rn > 0);
            }
        }
    }

    #[kani::proof]
    fn kani_rational_addition_does_not_panic() {
        let n1 = kani::any::<i128>();
        let d1 = kani::any::<i128>();
        let n2 = kani::any::<i128>();
        let d2 = kani::any::<i128>();
        kani::assume(d1 != 0);
        kani::assume(d2 != 0);

        if let (Ok(r1), Ok(r2)) = (Rational::new(n1, d1), Rational::new(n2, d2)) {
            // we just want to prove that addition doesn't panic
            let _ = r1.add(r2);
        }
    }
}
