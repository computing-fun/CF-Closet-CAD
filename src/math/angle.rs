use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Default, Debug, Clone, Copy)]
pub struct Angle {
    radians: Decimal,
}

impl Angle {
    pub fn from_radians(radians: Decimal) -> Self {
        Self { radians }
    }

    pub fn to_radians(self) -> Decimal {
        self.radians
    }

    pub fn from_degrees(degrees: Decimal) -> Self {
        Self {
            radians: (degrees * Decimal::PI / dec!(180)),
        }
    }

    pub fn to_degrees(self) -> Decimal {
        self.radians * dec!(180) / Decimal::PI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROUND_DP: u32 = 5;

    fn radians_to_degrees_tester(input_radians: Decimal, expected_degrees: Decimal) {
        let angle = Angle::from_radians(input_radians);
        let result = angle.to_degrees();
        assert_eq!(
            result.round_dp(ROUND_DP),
            expected_degrees.round_dp(ROUND_DP)
        )
    }

    fn degrees_to_radians_tester(input_degrees: Decimal, expected_radians: Decimal) {
        let angle = Angle::from_degrees(input_degrees);
        let result = angle.to_radians();
        assert_eq!(
            result.round_dp(ROUND_DP),
            expected_radians.round_dp(ROUND_DP)
        )
    }

    #[test]
    fn radians_to_degrees() {
        radians_to_degrees_tester(Decimal::ZERO, Decimal::ZERO);
        radians_to_degrees_tester(Decimal::PI / dec!(6), dec!(30));
        radians_to_degrees_tester(Decimal::QUARTER_PI, dec!(45));
        radians_to_degrees_tester(Decimal::PI / dec!(3), dec!(60));
        radians_to_degrees_tester(Decimal::HALF_PI, dec!(90));
        radians_to_degrees_tester(Decimal::TWO_PI / dec!(3), dec!(120));
        radians_to_degrees_tester(dec!(3) * Decimal::QUARTER_PI, dec!(135));
        radians_to_degrees_tester(dec!(5) * Decimal::PI / dec!(6), dec!(150));
        radians_to_degrees_tester(Decimal::PI, dec!(180));
        radians_to_degrees_tester(dec!(3) * Decimal::HALF_PI, dec!(270));
        radians_to_degrees_tester(Decimal::TWO_PI, dec!(360));
    }

    #[test]
    fn degrees_to_radians() {
        degrees_to_radians_tester(Decimal::ZERO, Decimal::ZERO);
        degrees_to_radians_tester(dec!(30), Decimal::PI / dec!(6));
        degrees_to_radians_tester(dec!(45), Decimal::QUARTER_PI);
        degrees_to_radians_tester(dec!(60), Decimal::PI / dec!(3));
        degrees_to_radians_tester(dec!(90), Decimal::HALF_PI);
        degrees_to_radians_tester(dec!(120), Decimal::TWO_PI / dec!(3));
        degrees_to_radians_tester(dec!(135), dec!(3) * Decimal::QUARTER_PI);
        degrees_to_radians_tester(dec!(150), dec!(5) * Decimal::PI / dec!(6));
        degrees_to_radians_tester(dec!(180), Decimal::PI);
        degrees_to_radians_tester(dec!(270), dec!(3) * Decimal::HALF_PI);
        degrees_to_radians_tester(dec!(360), Decimal::TWO_PI);
    }
}
