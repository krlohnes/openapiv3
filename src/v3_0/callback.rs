use crate::v3_0::*;
use indexmap::IndexMap;

/// A map of possible out-of band callbacks related to the parent operation.
/// Each value in the map is a Path Item Object that describes a set of
/// requests that may be initiated by the API provider and the expected
/// responses. The key value used to identify the callback object is an
/// expression, evaluated at runtime, that identifies a URL to use for the
/// callback operation.
pub type Callback = IndexMap<String, PathItem>;

#[cfg(feature = "conversions")]
use crate::v3_1;

#[cfg(feature = "conversions")]
pub fn callback_from_v3_1(a: IndexMap<String, v3_1::ReferenceOr<v3_1::PathItem>>) -> Callback {
    a.into_iter()
        .map(|(k, v)| {
            (
                k,
                match v {
                    v3_1::ReferenceOr::Reference { .. } => {
                        panic!("PathItems cannot be $ref")
                    }
                    v3_1::ReferenceOr::Item(item) => item.into(),
                    v3_1::ReferenceOr::DereferencedReference { item, .. } => item.into(),
                },
            )
        })
        .collect()
}
