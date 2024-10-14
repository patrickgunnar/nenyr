use indexmap::IndexMap;

use crate::{
    converters::{property::NenyrPropertyConverter, style_pattern::NenyrStylePatternConverter},
    validators::{identifier::NenyrIdentifierValidator, style_syntax::NenyrStyleSyntaxValidator},
};

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrStyleClass {
    class_name: Option<String>,
    deriving_from: Option<String>,
    is_important: Option<bool>,

    style_patterns: Option<IndexMap<String, IndexMap<String, String>>>,
    responsive_patterns: Option<IndexMap<String, IndexMap<String, IndexMap<String, String>>>>,
}

impl NenyrIdentifierValidator for NenyrStyleClass {}
impl NenyrStyleSyntaxValidator for NenyrStyleClass {}
impl NenyrPropertyConverter for NenyrStyleClass {}
impl NenyrStylePatternConverter for NenyrStyleClass {}

impl NenyrStyleClass {
    pub fn new() -> Self {
        Self {
            class_name: None,
            deriving_from: None,
            is_important: None,
            style_patterns: None,
            responsive_patterns: None,
        }
    }

    pub fn process_class(&self) {}
}
