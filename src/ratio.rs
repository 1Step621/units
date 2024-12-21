use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Ratio<const NUM: i32, const DEN: i32 = 1> {
    // HACK: prevent `Ratio` from being instantiated
    _do_not_instantiate: PhantomData<()>,
}

pub trait RatioTrait {
    const NUM: i32;
    const DEN: i32;
}
impl<const NUM: i32, const DEN: i32> RatioTrait for Ratio<NUM, DEN> {
    const NUM: i32 = NUM;
    const DEN: i32 = DEN;
}

pub const fn gcd(lhs: i32, rhs: i32) -> i32 {
    if rhs == 0 { lhs } else { gcd(rhs, lhs % rhs) }
}

pub const fn sign(x: i32) -> i32 {
    if x < 0 { -1 } else { 1 }
}

pub type Mul<Id1: RatioTrait, Id2: RatioTrait> = Ratio<
    {
        (Id1::NUM / gcd(Id1::NUM, Id2::DEN))
            * (Id2::NUM / gcd(Id2::NUM, Id1::DEN))
            * sign(Id1::DEN * Id2::DEN)
    },
    { ((Id1::DEN / gcd(Id2::NUM, Id1::DEN)) * (Id2::DEN / gcd(Id1::NUM, Id2::DEN))).abs() },
>;
pub type Div<Id1: RatioTrait, Id2: RatioTrait> = Ratio<
    {
        (Id1::NUM / gcd(Id1::NUM, Id2::NUM))
            * (Id2::DEN / gcd(Id2::DEN, Id1::DEN))
            * sign(Id1::DEN * Id2::NUM)
    },
    { ((Id1::DEN / gcd(Id1::DEN, Id2::DEN)) * (Id2::NUM / gcd(Id2::NUM, Id1::NUM))).abs() },
>;
