pub trait FloatConst {
    /// Euler's number (e).
    const E: Self;

    /// 1/π.
    const FRAC_1_PI: Self;

    /// 1/sqrt(2).
    const FRAC_1_SQRT_2: Self;

    /// 2/π
    const FRAC_2_PI: Self;

    /// 2/sqrt(π).
    const FRAC_2_SQRT_PI: Self;

    /// π/2.
    const FRAC_PI_2: Self;

    /// π/3.
    const FRAC_PI_3: Self;

    /// π/4.
    const FRAC_PI_4: Self;

    /// π/6.
    const FRAC_PI_6: Self;

    /// π/8.
    const FRAC_PI_8: Self;

    /// ln(2).
    const LN_2: Self;

    /// ln(10).
    const LN_10: Self;

    /// log2(10).
    const LOG2_10: Self;

    /// log2(e).
    const LOG2_E: Self;

    /// log10(2).
    const LOG10_2: Self;

    /// log10(e).
    const LOG10_E: Self;

    /// Archimedes' constant (π).
    const PI: Self;

    /// sqrt(2).
    const SQRT_2: Self;

    /// The full circle constant (τ).
    const TAU: Self;
}

macro_rules! impl_float_const {
    ($($t:ident )*) => {
        $(
            impl FloatConst for $t {
                const E: Self = std::$t::consts::E;

                const FRAC_1_PI: Self = std::$t::consts::FRAC_1_PI;

                const FRAC_1_SQRT_2: Self = std::$t::consts::FRAC_1_SQRT_2;

                const FRAC_2_PI: Self = std::$t::consts::FRAC_2_PI;

                const FRAC_2_SQRT_PI: Self = std::$t::consts::FRAC_2_SQRT_PI;

                const FRAC_PI_2: Self = std::$t::consts::FRAC_PI_2;

                const FRAC_PI_3: Self = std::$t::consts::FRAC_PI_3;

                const FRAC_PI_4: Self = std::$t::consts::FRAC_PI_4;

                const FRAC_PI_6: Self = std::$t::consts::FRAC_PI_6;

                const FRAC_PI_8: Self = std::$t::consts::FRAC_PI_8;

                const LN_2: Self = std::$t::consts::LN_2;

                const LN_10: Self = std::$t::consts::LN_10;

                const LOG2_10: Self = std::$t::consts::LOG2_10;

                const LOG2_E: Self = std::$t::consts::LOG2_E;

                const LOG10_2: Self = std::$t::consts::LOG10_2;

                const LOG10_E: Self = std::$t::consts::LOG10_E;

                const PI: Self = std::$t::consts::PI;

                const SQRT_2: Self = std::$t::consts::SQRT_2;

                const TAU: Self = std::$t::consts::TAU;
            }
        )*
    };
}

impl_float_const!(f32 f64);
