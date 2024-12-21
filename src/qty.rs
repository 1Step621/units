use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use num::Num;

use crate::{
    ratio::RatioTrait,
    unit::{self, UnitTrait},
};

#[derive(Clone, Copy)]
pub struct Qty<Value: Num, Unit: UnitTrait>(Value, PhantomData<Unit>);

impl<Value: Num, Unit: UnitTrait> Qty<Value, Unit> {
    pub fn new(value: Value) -> Self {
        Self(value, PhantomData)
    }
}

impl<Value: Num, Unit: UnitTrait> Add<Qty<Value, Unit>> for Qty<Value, Unit> {
    type Output = Qty<Value, Unit>;

    fn add(self, rhs: Qty<Value, Unit>) -> Self::Output {
        Qty(self.0 + rhs.0, PhantomData)
    }
}

impl<Value: Num, Unit: UnitTrait> Sub<Qty<Value, Unit>> for Qty<Value, Unit> {
    type Output = Qty<Value, Unit>;

    fn sub(self, rhs: Qty<Value, Unit>) -> Self::Output {
        Qty(self.0 - rhs.0, PhantomData)
    }
}

impl<Value: Num, Unit1: UnitTrait, Unit2: UnitTrait> Mul<Qty<Value, Unit2>> for Qty<Value, Unit1>
where
    unit::Mul<Unit1, Unit2>: UnitTrait,
{
    type Output = Qty<Value, unit::Mul<Unit1, Unit2>>;

    fn mul(self, rhs: Qty<Value, Unit2>) -> Self::Output {
        Qty(self.0 * rhs.0, PhantomData)
    }
}

impl<Value: Num, Unit1: UnitTrait, Unit2: UnitTrait> Div<Qty<Value, Unit2>> for Qty<Value, Unit1>
where
    unit::Div<Unit1, Unit2>: UnitTrait,
{
    type Output = Qty<Value, unit::Div<Unit1, Unit2>>;

    fn div(self, rhs: Qty<Value, Unit2>) -> Self::Output {
        Qty(self.0 / rhs.0, PhantomData)
    }
}

impl<Value: Num + Display, Unit: UnitTrait> Display for Qty<Value, Unit> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{} / {}]", self.0, Unit::Id::NUM, Unit::Id::DEN)
    }
}
