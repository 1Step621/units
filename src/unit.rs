use std::marker::PhantomData;

use crate::ratio::{self, RatioTrait};

#[derive(Clone, Copy)]
pub struct Unit<Id: RatioTrait>(PhantomData<Id>);

pub trait UnitTrait {
    type Id: RatioTrait;
}
impl<Id: RatioTrait> UnitTrait for Unit<Id> {
    type Id = Id;
}

pub type Mul<Unit1: UnitTrait, Unit2: UnitTrait> = Unit<ratio::Mul<Unit1::Id, Unit2::Id>>;
pub type Div<Unit1: UnitTrait, Unit2: UnitTrait> = Unit<ratio::Div<Unit1::Id, Unit2::Id>>;
