#![allow(non_camel_case_types)]

pub use SpecId::*;

/// Specification IDs and their activation block.
///
/// Information was obtained from the [Ethereum Execution Specifications](https://github.com/ethereum/execution-specs)
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, enumn::N)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpecId {
    FRONTIER = 0,         // Frontier               0
    FRONTIER_THAWING = 1, // Frontier Thawing       200000
    HOMESTEAD = 2,        // Homestead              1150000
    DAO_FORK = 3,         // DAO Fork               1920000
    TANGERINE = 4,        // Tangerine Whistle      2463000
    SPURIOUS_DRAGON = 5,  // Spurious Dragon        2675000
    BYZANTIUM = 6,        // Byzantium              4370000
    CONSTANTINOPLE = 7,   // Constantinople         7280000 is overwritten with PETERSBURG
    PETERSBURG = 8,       // Petersburg             7280000
    ISTANBUL = 9,         // Istanbul	            9069000
    MUIR_GLACIER = 10,    // Muir Glacier           9200000
    BERLIN = 11,          // Berlin	                12244000
    LONDON = 12,          // London	                12965000
    ARROW_GLACIER = 13,   // Arrow Glacier          13773000
    GRAY_GLACIER = 14,    // Gray Glacier           15050000
    MERGE = 15,           // Paris/Merge            15537394 (TTD: 58750000000000000000000)
    SHANGHAI = 16,        // Shanghai               17034870 (Timestamp: 1681338455)
    CANCUN = 17,          // Cancun                 19426587 (Timestamp: 1710338135)
    PRAGUE = 18,          // Praque                 TBD
    #[default]
    LATEST = u8::MAX,
}

impl SpecId {
    /// Returns the `SpecId` for the given `u8`.
    #[inline]
    pub fn try_from_u8(spec_id: u8) -> Option<Self> {
        Self::n(spec_id)
    }

    /// Returns `true` if the given specification ID is enabled in this spec.
    #[inline]
    pub const fn is_enabled_in(self, other: Self) -> bool {
        Self::enabled(self, other)
    }

    /// Returns `true` if the given specification ID is enabled in this spec.
    #[inline]
    pub const fn enabled(our: SpecId, other: SpecId) -> bool {
        our as u8 >= other as u8
    }
}

impl From<&str> for SpecId {
    fn from(name: &str) -> Self {
        match name {
            "Frontier" => Self::FRONTIER,
            "Homestead" => Self::HOMESTEAD,
            "Tangerine" => Self::TANGERINE,
            "Spurious" => Self::SPURIOUS_DRAGON,
            "Byzantium" => Self::BYZANTIUM,
            "Constantinople" => Self::CONSTANTINOPLE,
            "Petersburg" => Self::PETERSBURG,
            "Istanbul" => Self::ISTANBUL,
            "MuirGlacier" => Self::MUIR_GLACIER,
            "Berlin" => Self::BERLIN,
            "London" => Self::LONDON,
            "Merge" => Self::MERGE,
            "Shanghai" => Self::SHANGHAI,
            "Cancun" => Self::CANCUN,
            "Prague" => Self::PRAGUE,
            _ => Self::LATEST,
        }
    }
}

impl From<SpecId> for &'static str {
    fn from(spec_id: SpecId) -> Self {
        match spec_id {
            SpecId::FRONTIER => "Frontier",
            SpecId::FRONTIER_THAWING => "Frontier Thawing",
            SpecId::HOMESTEAD => "Homestead",
            SpecId::DAO_FORK => "DAO Fork",
            SpecId::TANGERINE => "Tangerine",
            SpecId::SPURIOUS_DRAGON => "Spurious",
            SpecId::BYZANTIUM => "Byzantium",
            SpecId::CONSTANTINOPLE => "Constantinople",
            SpecId::PETERSBURG => "Petersburg",
            SpecId::ISTANBUL => "Istanbul",
            SpecId::MUIR_GLACIER => "MuirGlacier",
            SpecId::BERLIN => "Berlin",
            SpecId::LONDON => "London",
            SpecId::ARROW_GLACIER => "Arrow Glacier",
            SpecId::GRAY_GLACIER => "Gray Glacier",
            SpecId::MERGE => "Merge",
            SpecId::SHANGHAI => "Shanghai",
            SpecId::CANCUN => "Cancun",
            SpecId::PRAGUE => "Prague",
            SpecId::LATEST => "Latest",
        }
    }
}

#[macro_export]
macro_rules! impl_chain_spec {
    ($spec_ty:ident, $(
        $spec_id:ident => $spec_name:ident,
    )+) => {
        pub trait Spec: Sized + 'static {
            /// The specification ID.
            const SPEC_ID: $spec_ty;

            /// Returns `true` if the given specification ID is enabled in this spec.
            #[inline]
            fn enabled(spec_id: $spec_ty) -> bool {
                $spec_ty::enabled(Self::SPEC_ID, spec_id)
            }
        }

        $(
            #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $spec_name;

            impl Spec for $spec_name {
                const SPEC_ID: $spec_ty = $spec_ty::$spec_id;
            }
        )+
    }
}

impl_chain_spec! {
    SpecId,
    FRONTIER => FrontierSpec,
    // FRONTIER_THAWING no EVM spec change
    HOMESTEAD => HomesteadSpec,
    // DAO_FORK no EVM spec change
    TANGERINE => TangerineSpec,
    SPURIOUS_DRAGON => SpuriousDragonSpec,
    BYZANTIUM => ByzantiumSpec,
    // CONSTANTINOPLE was overridden with PETERSBURG
    PETERSBURG => PetersburgSpec,
    ISTANBUL => IstanbulSpec,
    // MUIR_GLACIER no EVM spec change
    BERLIN => BerlinSpec,
    LONDON => LondonSpec,
    // ARROW_GLACIER no EVM spec change
    // GRAY_GLACIER no EVM spec change
    MERGE => MergeSpec,
    SHANGHAI => ShanghaiSpec,
    CANCUN => CancunSpec,
    PRAGUE => PragueSpec,

    LATEST => LatestSpec,
}

#[macro_export]
macro_rules! spec_to_generic {
    ($spec_id:expr, $e:expr) => {{
        // We are transitioning from var to generic spec.
        match $spec_id {
            $crate::SpecId::FRONTIER | SpecId::FRONTIER_THAWING => {
                use $crate::FrontierSpec as SPEC;
                $e
            }
            $crate::SpecId::HOMESTEAD | SpecId::DAO_FORK => {
                use $crate::HomesteadSpec as SPEC;
                $e
            }
            $crate::SpecId::TANGERINE => {
                use $crate::TangerineSpec as SPEC;
                $e
            }
            $crate::SpecId::SPURIOUS_DRAGON => {
                use $crate::SpuriousDragonSpec as SPEC;
                $e
            }
            $crate::SpecId::BYZANTIUM => {
                use $crate::ByzantiumSpec as SPEC;
                $e
            }
            $crate::SpecId::PETERSBURG | $crate::SpecId::CONSTANTINOPLE => {
                use $crate::PetersburgSpec as SPEC;
                $e
            }
            $crate::SpecId::ISTANBUL | $crate::SpecId::MUIR_GLACIER => {
                use $crate::IstanbulSpec as SPEC;
                $e
            }
            $crate::SpecId::BERLIN => {
                use $crate::BerlinSpec as SPEC;
                $e
            }
            $crate::SpecId::LONDON
            | $crate::SpecId::ARROW_GLACIER
            | $crate::SpecId::GRAY_GLACIER => {
                use $crate::LondonSpec as SPEC;
                $e
            }
            $crate::SpecId::MERGE => {
                use $crate::MergeSpec as SPEC;
                $e
            }
            $crate::SpecId::SHANGHAI => {
                use $crate::ShanghaiSpec as SPEC;
                $e
            }
            $crate::SpecId::CANCUN => {
                use $crate::CancunSpec as SPEC;
                $e
            }
            $crate::SpecId::LATEST => {
                use $crate::LatestSpec as SPEC;
                $e
            }
            $crate::SpecId::PRAGUE => {
                use $crate::PragueSpec as SPEC;
                $e
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_to_generic() {
        use SpecId::*;

        spec_to_generic!(FRONTIER, assert_eq!(SPEC::SPEC_ID, FRONTIER));
        spec_to_generic!(FRONTIER_THAWING, assert_eq!(SPEC::SPEC_ID, FRONTIER));
        spec_to_generic!(HOMESTEAD, assert_eq!(SPEC::SPEC_ID, HOMESTEAD));
        spec_to_generic!(DAO_FORK, assert_eq!(SPEC::SPEC_ID, HOMESTEAD));
        spec_to_generic!(TANGERINE, assert_eq!(SPEC::SPEC_ID, TANGERINE));
        spec_to_generic!(SPURIOUS_DRAGON, assert_eq!(SPEC::SPEC_ID, SPURIOUS_DRAGON));
        spec_to_generic!(BYZANTIUM, assert_eq!(SPEC::SPEC_ID, BYZANTIUM));
        spec_to_generic!(CONSTANTINOPLE, assert_eq!(SPEC::SPEC_ID, PETERSBURG));
        spec_to_generic!(PETERSBURG, assert_eq!(SPEC::SPEC_ID, PETERSBURG));
        spec_to_generic!(ISTANBUL, assert_eq!(SPEC::SPEC_ID, ISTANBUL));
        spec_to_generic!(MUIR_GLACIER, assert_eq!(SPEC::SPEC_ID, ISTANBUL));
        spec_to_generic!(BERLIN, assert_eq!(SPEC::SPEC_ID, BERLIN));
        spec_to_generic!(LONDON, assert_eq!(SPEC::SPEC_ID, LONDON));
        spec_to_generic!(ARROW_GLACIER, assert_eq!(SPEC::SPEC_ID, LONDON));
        spec_to_generic!(GRAY_GLACIER, assert_eq!(SPEC::SPEC_ID, LONDON));
        spec_to_generic!(MERGE, assert_eq!(SPEC::SPEC_ID, MERGE));
        spec_to_generic!(SHANGHAI, assert_eq!(SPEC::SPEC_ID, SHANGHAI));
        spec_to_generic!(CANCUN, assert_eq!(SPEC::SPEC_ID, CANCUN));
        spec_to_generic!(PRAGUE, assert_eq!(SPEC::SPEC_ID, PRAGUE));
        spec_to_generic!(LATEST, assert_eq!(SPEC::SPEC_ID, LATEST));
    }
}
