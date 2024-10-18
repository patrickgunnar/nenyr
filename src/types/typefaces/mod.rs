use indexmap::IndexMap;

use crate::validators::{identifier::NenyrIdentifierValidator, typeface::NenyrTypefaceValidator};

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrTypefaces {
    values: IndexMap<String, String>,
}

impl NenyrIdentifierValidator for NenyrTypefaces {}
impl NenyrTypefaceValidator for NenyrTypefaces {}