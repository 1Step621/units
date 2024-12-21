use crate::ratio::Ratio;

use crate::unit::{self, Unit};

pub type Meter = Unit<Ratio<2>>;
pub type Kilogram = Unit<Ratio<3>>;
pub type Second = Unit<Ratio<5>>;
pub type Ampere = Unit<Ratio<7>>;
pub type Kelvin = Unit<Ratio<11>>;
pub type Mole = Unit<Ratio<13>>;
pub type Candela = Unit<Ratio<17>>;

pub type Dimless = Unit<Ratio<1>>;

// pub type SquareMeter = unit::Mul<Meter, Meter>;
// pub type CubicMeter = unit::Mul<SquareMeter, Meter>;
pub type MeterPerSecond = unit::Div<Meter, Second>;
pub type MeterPerSecondSquared = unit::Div<MeterPerSecond, Second>;
// pub type Hertz = unit::Div<Dimless, Second>;
// pub type KilogramMeterPerCubicMeter = unit::Div<Kilogram, CubicMeter>;
pub type Newton = unit::Mul<Kilogram, MeterPerSecondSquared>;
// pub type Pascal = unit::Div<Newton, SquareMeter>;
pub type Joule = unit::Mul<Newton, Meter>;
// pub type NewtonMeter = Joule;
// pub type KilogramSquareMeter = unit::Mul<Kilogram, SquareMeter>;
pub type Watt = unit::Div<Joule, Second>;
pub type Coulomb = unit::Mul<Ampere, Second>;
pub type Volt = unit::Div<Watt, Ampere>;
// pub type Farad = unit::Div<Coulomb, Volt>;
// pub type Ohm = unit::Div<Volt, Ampere>;
// pub type Siemens = unit::Div<Dimless, Ohm>;
// pub type Weber = unit::Mul<Volt, Second>;
// pub type Tesla = unit::Div<Weber, SquareMeter>;
// pub type Henry = unit::Div<Weber, Ampere>;
// pub type Gray = unit::Div<Joule, Kilogram>;
// pub type Sievert = Gray;
// pub type KilogramPerMole = unit::Div<Kilogram, Mole>;
// pub type MolePerCubicMeter = unit::Div<Mole, CubicMeter>;
// pub type CubicMeterPerMole = unit::Div<CubicMeter, Mole>;
// pub type Nit = unit::Mul<Candela, SquareMeter>;
