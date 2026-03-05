//! Define acc inputs from Ehorizon & maps dats
//!

use acc_interface::datatypes::{ahOut_t, AHorizonCountryCode, AHorizonDrivingSide};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

/// Driving side
#[derive(Debug, Clone, Default, PartialEq)]
pub enum DrivingSide {
    /// Left side
    Left,
    /// Right side
    Right,
    /// Unknown
    #[default]
    Unknown,
}
/// Ahorizon inputs
#[derive(Debug, Clone, Default)]
pub struct Ahorizon {
    /// driving side
    pub driving_side: DrivingSide,
    /// country code
    pub country_code: CountryCode,
}
impl Ahorizon {
    /// Check if driving side is left
    pub fn left_driving_side(&self) -> bool {
        matches!(self.driving_side, DrivingSide::Left)
    }
    /// Check if driving side is right
    pub fn right_driving_side(&self) -> bool {
        matches!(self.driving_side, DrivingSide::Right)
    }
    /// Check if driving side is left or Unknown
    pub fn left_driving_side_or_unknown(&self) -> bool {
        matches!(self.driving_side, DrivingSide::Left | DrivingSide::Unknown)
    }
    /// Check if driving side is right or Unknown
    pub fn right_driving_side_or_unknown(&self) -> bool {
        matches!(self.driving_side, DrivingSide::Right | DrivingSide::Unknown)
    }

    /// is anti_undertake allowed
    // Req: SCRS-1182790_4
    pub fn is_undertake_allowed(&self) -> bool {
        !matches!(
            self.country_code,
            CountryCode::Austria
                | CountryCode::Bulgaria
                | CountryCode::Poland
                | CountryCode::Uk
                | CountryCode::Usa
                | CountryCode::Unknown
        ) && self.driving_side.ne(&DrivingSide::Unknown)
    }
}

impl From<AHorizonDrivingSide> for DrivingSide {
    fn from(ah: AHorizonDrivingSide) -> Self {
        match ah {
            AHorizonDrivingSide::AHDS_LEFT => DrivingSide::Left,
            AHorizonDrivingSide::AHDS_RIGHT => DrivingSide::Right,
            AHorizonDrivingSide::AHDS_UNKNOWN => DrivingSide::Unknown,
        }
    }
}

/// Represents a country code.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum CountryCode {
    /// Unknown
    #[default]
    Unknown,
    /// Albania
    Albania,
    /// Algeria
    Algeria,
    /// Andorra
    Andorra,
    /// Austria
    Austria,
    /// Belgium
    Belgium,
    /// Bosnia
    Bosnia,
    /// Bulgaria
    Bulgaria,
    /// Bielorussia
    Bielorussia,
    /// Kosovo
    Kosovo,
    /// China
    China,
    /// Mayotte
    Mayotte,
    /// Croatia
    Croatia,
    /// Cyprus
    Cyprus,
    /// Czech
    Czech,
    /// Denmark
    Denmark,
    /// Estonia
    Estonia,
    /// Finland
    Finland,
    /// France
    France,
    /// Guyana
    Guyana,
    /// French Guiana
    FenchGuyana,
    /// Polynesia
    Polynesia,
    /// Germany
    Germany,
    /// Gibraltar
    Gibraltar,
    /// Greece
    Greece,
    /// Guadeloupe
    Guadeloupe,
    /// Vatican
    Vatican,
    /// Hungary
    Hungary,
    /// Ireland
    Ireland,
    /// Israel
    Israel,
    /// Italy
    Italy,
    /// India,
    India,
    /// Japan
    Japan,
    /// Korea (South)
    Koreasouth,
    /// Latvia
    Latvia,
    /// Liechtenstein
    Liechtenstein,
    /// Lithuania
    Lithuania,
    /// Luxembourg
    Luxembourg,
    /// Malta
    Malta,
    /// Martinique
    Martinique,
    /// Monaco
    Monaco,
    /// Moldavia
    Moldavia,
    /// Montenegro
    Montenegro,
    /// Morocco
    Morocco,
    /// Netherlands
    Netherlands,
    /// Aruba
    Aruba,
    /// St. Marteen
    StMarteen,
    /// Bonaire
    Bonaire,
    /// New Caledonia
    NewCaledonia,
    /// Norway
    Norway,
    /// Poland
    Poland,
    /// Portugal
    Portugal,
    /// Réunion
    Reunion,
    /// Romania
    Romania,
    /// Russia
    Russia,
    /// St. Barth
    StBarth,
    /// St. Martin
    StMartin,
    /// St. Pierre
    StPierre,
    /// San Marino
    SanMarino,
    /// Serbia
    Serbia,
    /// Slovakia
    Slovakia,
    /// Slovenia
    Slovenia,
    /// Spain
    Spain,
    /// Sweden
    Sweden,
    /// Switzerland
    Switzerland,
    /// Syria
    Syria,
    /// Tunisia
    Tunisia,
    /// Turkey
    Turkey,
    /// Ukrainia
    Ukrainia,
    /// Macedonia
    Macedonia,
    /// United Kingdom
    Uk,
    /// Guernesey
    Guernesey,
    /// Jersey
    Jersey,
    /// United States (USA)
    Usa,
    /// Wallis
    Wallis,
    /// Mask (Special / Reserved)
    Mask,
    /// Guiana
    Guiana,
}

/// Converts from `AHorizonCountryCode` to `SimpleCountryCode`.
impl From<AHorizonCountryCode> for CountryCode {
    fn from(value: AHorizonCountryCode) -> Self {
        match value {
            AHorizonCountryCode::AHCC_UNKNOWN => Self::Unknown,
            AHorizonCountryCode::AHCC_ALBANIA => Self::Albania,
            AHorizonCountryCode::AHCC_ALGERIA => Self::Algeria,
            AHorizonCountryCode::AHCC_ANDORRA => Self::Andorra,
            AHorizonCountryCode::AHCC_AUSTRIA => Self::Austria,
            AHorizonCountryCode::AHCC_BELGIUM => Self::Belgium,
            AHorizonCountryCode::AHCC_BOSNIA => Self::Bosnia,
            AHorizonCountryCode::AHCC_BULGARIA => Self::Bulgaria,
            AHorizonCountryCode::AHCC_BIELORUSSIA => Self::Bielorussia,
            AHorizonCountryCode::AHCC_KOSOVO => Self::Kosovo,
            AHorizonCountryCode::AHCC_CHINA => Self::China,
            AHorizonCountryCode::AHCC_MAYOTTE => Self::Mayotte,
            AHorizonCountryCode::AHCC_CROATIA => Self::Croatia,
            AHorizonCountryCode::AHCC_CYPRUS => Self::Cyprus,
            AHorizonCountryCode::AHCC_CZECH => Self::Czech,
            AHorizonCountryCode::AHCC_DENMARK => Self::Denmark,
            AHorizonCountryCode::AHCC_ESTONIA => Self::Estonia,
            AHorizonCountryCode::AHCC_FINLAND => Self::Finland,
            AHorizonCountryCode::AHCC_FRANCE => Self::France,
            AHorizonCountryCode::AHCC_GUYANA => Self::Guyana,
            AHorizonCountryCode::AHCC_FRENCH_GUIANA => Self::FenchGuyana,
            AHorizonCountryCode::AHCC_POLYNESIA => Self::Polynesia,
            AHorizonCountryCode::AHCC_GERMANY => Self::Germany,
            AHorizonCountryCode::AHCC_GIBRALTAR => Self::Gibraltar,
            AHorizonCountryCode::AHCC_GREECE => Self::Greece,
            AHorizonCountryCode::AHCC_GUADELOUPE => Self::Guadeloupe,
            AHorizonCountryCode::AHCC_VATICAN => Self::Vatican,
            AHorizonCountryCode::AHCC_HUNGARY => Self::Hungary,
            AHorizonCountryCode::AHCC_IRELAND => Self::Ireland,
            AHorizonCountryCode::AHCC_ISRAEL => Self::Israel,
            AHorizonCountryCode::AHCC_ITALY => Self::Italy,
            AHorizonCountryCode::AHCC_JAPAN => Self::Japan,
            AHorizonCountryCode::AHCC_KOREASOUTH => Self::Koreasouth,
            AHorizonCountryCode::AHCC_LATVIA => Self::Latvia,
            AHorizonCountryCode::AHCC_LIECHTENSTEIN => Self::Liechtenstein,
            AHorizonCountryCode::AHCC_LITHUANIA => Self::Lithuania,
            AHorizonCountryCode::AHCC_LUXEMBOURG => Self::Luxembourg,
            AHorizonCountryCode::AHCC_MALTA => Self::Malta,
            AHorizonCountryCode::AHCC_MARTINIQUE => Self::Martinique,
            AHorizonCountryCode::AHCC_MONACO => Self::Monaco,
            AHorizonCountryCode::AHCC_MOLDAVIA => Self::Moldavia,
            AHorizonCountryCode::AHCC_MONTENEGRO => Self::Montenegro,
            AHorizonCountryCode::AHCC_MOROCCO => Self::Morocco,
            AHorizonCountryCode::AHCC_NETHERLANDS => Self::Netherlands,
            AHorizonCountryCode::AHCC_ARUBA => Self::Aruba,
            AHorizonCountryCode::AHCC_ST_MARTEEN => Self::StMarteen,
            AHorizonCountryCode::AHCC_BONAIRE => Self::Bonaire,
            AHorizonCountryCode::AHCC_NEW_CALEDONIA => Self::NewCaledonia,
            AHorizonCountryCode::AHCC_NORWAY => Self::Norway,
            AHorizonCountryCode::AHCC_POLAND => Self::Poland,
            AHorizonCountryCode::AHCC_PORTUGAL => Self::Portugal,
            AHorizonCountryCode::AHCC_REUNION => Self::Reunion,
            AHorizonCountryCode::AHCC_ROMANIA => Self::Romania,
            AHorizonCountryCode::AHCC_RUSSIA => Self::Russia,
            AHorizonCountryCode::AHCC_ST_BARTH => Self::StBarth,
            AHorizonCountryCode::AHCC_ST_MARTIN => Self::StMartin,
            AHorizonCountryCode::AHCC_ST_PIERRE => Self::StPierre,
            AHorizonCountryCode::AHCC_SAN_MARINO => Self::SanMarino,
            AHorizonCountryCode::AHCC_SERBIA => Self::Serbia,
            AHorizonCountryCode::AHCC_SLOVAKIA => Self::Slovakia,
            AHorizonCountryCode::AHCC_SLOVENIA => Self::Slovenia,
            AHorizonCountryCode::AHCC_SPAIN => Self::Spain,
            AHorizonCountryCode::AHCC_SWEDEN => Self::Sweden,
            AHorizonCountryCode::AHCC_SWITZERLAND => Self::Switzerland,
            AHorizonCountryCode::AHCC_SYRIA => Self::Syria,
            AHorizonCountryCode::AHCC_TUNISIA => Self::Tunisia,
            AHorizonCountryCode::AHCC_TURKEY => Self::Turkey,
            AHorizonCountryCode::AHCC_UKRAINA => Self::Ukrainia,
            AHorizonCountryCode::AHCC_MACEDONIA => Self::Macedonia,
            AHorizonCountryCode::AHCC_UK => Self::Uk,
            AHorizonCountryCode::AHCC_GUERNESEY => Self::Guernesey,
            AHorizonCountryCode::AHCC_JERSEY => Self::Jersey,
            AHorizonCountryCode::AHCC_USA => Self::Usa,
            AHorizonCountryCode::AHCC_WALLIS => Self::Wallis,
            AHorizonCountryCode::AHCC_INDIA => Self::India,
            AHorizonCountryCode::AHCC_MASK => Self::Mask,
        }
    }
}

impl From<&ahOut_t> for Ahorizon {
    fn from(ah: &ahOut_t) -> Self {
        Self {
            driving_side: ah.meta_data.driving_side.into(),
            country_code: ah.meta_data.country_code.into(),
        }
    }
}
#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ahr_types::AhOut_t> for Ahorizon {
    fn from(ah: &oem::sdv_adas_ahr_types::AhOut_t) -> Self {
        Self {
            driving_side: ah.meta_data.driving_side.enum_value_or_default().into(),
            country_code: ah.meta_data.country_code.enum_value_or_default().into(),
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ahr_types::AHorizonDrivingSide> for DrivingSide {
    fn from(value: oem::sdv_adas_ahr_types::AHorizonDrivingSide) -> Self {
        use oem::sdv_adas_ahr_types::AHorizonDrivingSide::*;
        match value {
            AHDS_LEFT => DrivingSide::Left,
            AHDS_RIGHT => DrivingSide::Right,
            AHDS_UNKNOWN => DrivingSide::Unknown,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ahr_types::AHorizonCountryCode> for CountryCode {
    fn from(value: oem::sdv_adas_ahr_types::AHorizonCountryCode) -> Self {
        use oem::sdv_adas_ahr_types::AHorizonCountryCode::*;
        match value {
            AHCC_UNKNOWN => Self::Unknown,
            AHCC_ALBANIA => Self::Albania,
            AHCC_ALGERIA => Self::Algeria,
            AHCC_ANDORRA => Self::Andorra,
            AHCC_AUSTRIA => Self::Austria,
            AHCC_BELGIUM => Self::Belgium,
            AHCC_BOSNIA => Self::Bosnia,
            AHCC_BULGARIA => Self::Bulgaria,
            AHCC_BIELORUSSIA => Self::Bielorussia,
            AHCC_KOSOVO => Self::Kosovo,
            AHCC_CHINA => Self::China,
            AHCC_MAYOTTE => Self::Mayotte,
            AHCC_CROATIA => Self::Croatia,
            AHCC_CYPRUS => Self::Cyprus,
            AHCC_CZECH => Self::Czech,
            AHCC_DENMARK => Self::Denmark,
            AHCC_ESTONIA => Self::Estonia,
            AHCC_FINLAND => Self::Finland,
            AHCC_FRANCE => Self::France,
            AHCC_FRENCH_GUIANA => Self::Guyana,
            AHCC_GUYANA => Self::Guyana,
            AHCC_POLYNESIA => Self::Polynesia,
            AHCC_GERMANY => Self::Germany,
            AHCC_GIBRALTAR => Self::Gibraltar,
            AHCC_GREECE => Self::Greece,
            AHCC_GUADELOUPE => Self::Guadeloupe,
            AHCC_VATICAN => Self::Vatican,
            AHCC_HUNGARY => Self::Hungary,
            AHCC_IRELAND => Self::Ireland,
            AHCC_ISRAEL => Self::Israel,
            AHCC_ITALY => Self::Italy,
            AHCC_JAPAN => Self::Japan,
            AHCC_KOREASOUTH => Self::Koreasouth,
            AHCC_LATVIA => Self::Latvia,
            AHCC_LIECHTENSTEIN => Self::Liechtenstein,
            AHCC_LITHUANIA => Self::Lithuania,
            AHCC_LUXEMBOURG => Self::Luxembourg,
            AHCC_MALTA => Self::Malta,
            AHCC_MARTINIQUE => Self::Martinique,
            AHCC_MONACO => Self::Monaco,
            AHCC_MOLDAVIA => Self::Moldavia,
            AHCC_MONTENEGRO => Self::Montenegro,
            AHCC_MOROCCO => Self::Morocco,
            AHCC_NETHERLANDS => Self::Netherlands,
            AHCC_ARUBA => Self::Aruba,
            AHCC_ST_MARTEEN => Self::StMarteen,
            AHCC_BONAIRE => Self::Bonaire,
            AHCC_NEW_CALEDONIA => Self::NewCaledonia,
            AHCC_NORWAY => Self::Norway,
            AHCC_POLAND => Self::Poland,
            AHCC_PORTUGAL => Self::Portugal,
            AHCC_REUNION => Self::Reunion,
            AHCC_ROMANIA => Self::Romania,
            AHCC_RUSSIA => Self::Russia,
            AHCC_ST_BARTH => Self::StBarth,
            AHCC_ST_MARTIN => Self::StMartin,
            AHCC_ST_PIERRE => Self::StPierre,
            AHCC_SAN_MARINO => Self::SanMarino,
            AHCC_SERBIA => Self::Serbia,
            AHCC_SLOVAKIA => Self::Slovakia,
            AHCC_SLOVENIA => Self::Slovenia,
            AHCC_SPAIN => Self::Spain,
            AHCC_SWEDEN => Self::Sweden,
            AHCC_SWITZERLAND => Self::Switzerland,
            AHCC_SYRIA => Self::Syria,
            AHCC_TUNISIA => Self::Tunisia,
            AHCC_TURKEY => Self::Turkey,
            AHCC_UKRAINA => Self::Ukrainia,
            AHCC_MACEDONIA => Self::Macedonia,
            AHCC_UK => Self::Uk,
            AHCC_GUERNESEY => Self::Guernesey,
            AHCC_JERSEY => Self::Jersey,
            AHCC_USA => Self::Usa,
            AHCC_WALLIS => Self::Wallis,
            AHCC_INDIA => Self::India,
            AHCC_MASK => Self::Mask,
        }
    }
}
