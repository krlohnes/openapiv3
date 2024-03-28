use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Item(T),
    DereferencedReference {
        reference: String,
        item: T,
    },
}

impl<T> ReferenceOr<T> {
    pub fn ref_(r: &str) -> Self {
        ReferenceOr::Reference {
            reference: r.to_owned(),
        }
    }
    pub fn boxed_item(item: T) -> ReferenceOr<Box<T>> {
        ReferenceOr::Item(Box::new(item))
    }

    /// Converts this [ReferenceOr] to the item inside, if it exists.
    ///
    /// The return value will be [Option::Some] if this was a
    /// [ReferenceOr::Item] or [Option::None] if this was a
    /// [ReferenceOr::Reference].
    ///
    /// # Examples
    ///
    /// ```
    /// # use openapiv3::v3_0::ReferenceOr;
    ///
    /// let i = ReferenceOr::Item(1);
    /// assert_eq!(i.into_item(), Some(1));
    ///
    /// let j: ReferenceOr<u8> = ReferenceOr::Reference { reference: String::new() };
    /// assert_eq!(j.into_item(), None);
    /// ```
    pub fn into_item(self) -> Option<T> {
        match self {
            ReferenceOr::Reference { .. } => None,
            ReferenceOr::Item(i) => Some(i),
            ReferenceOr::DereferencedReference { reference: _, item } => Some(item),
        }
    }

    /// Returns a reference to to the item inside this [ReferenceOr], if it
    /// exists.
    ///
    /// The return value will be [Option::Some] if this was a
    /// [ReferenceOr::Item] or [Option::None] if this was a
    /// [ReferenceOr::Reference].
    ///
    /// # Examples
    ///
    /// ```
    /// # use openapiv3::v3_0::ReferenceOr;
    ///
    /// let i = ReferenceOr::Item(1);
    /// assert_eq!(i.as_item(), Some(&1));
    ///
    /// let j: ReferenceOr<u8> = ReferenceOr::Reference { reference: String::new() };
    /// assert_eq!(j.as_item(), None);
    /// ```
    pub fn as_item(&self) -> Option<&T> {
        match self {
            ReferenceOr::Reference { .. } => None,
            ReferenceOr::Item(i) => Some(i),
            ReferenceOr::DereferencedReference { reference: _, item } => Some(item),
        }
    }
}

impl<T> ReferenceOr<Box<T>> {
    pub fn unbox(self) -> ReferenceOr<T> {
        match self {
            ReferenceOr::Reference { reference } => ReferenceOr::Reference { reference },
            ReferenceOr::Item(boxed) => ReferenceOr::Item(*boxed),
            ReferenceOr::DereferencedReference { reference, item } => {
                ReferenceOr::DereferencedReference {
                    reference,
                    item: *item,
                }
            }
        }
    }
}

#[cfg(feature = "conversions")]
use crate::v3_1;

#[cfg(feature = "conversions")]
impl<T> ReferenceOr<T> {
    pub fn from_v3_1<X: Into<T>>(r: v3_1::ReferenceOr<X>) -> Self {
        match r {
            v3_1::ReferenceOr::Reference { reference, .. } => ReferenceOr::Reference { reference },
            v3_1::ReferenceOr::Item(item) => ReferenceOr::Item(item.into()),
            //This could go one way or the other. Item suits my curren needs better.
            v3_1::ReferenceOr::DereferencedReference {
                reference: _, item, ..
            } => ReferenceOr::Item(item.into()),
        }
    }
}
