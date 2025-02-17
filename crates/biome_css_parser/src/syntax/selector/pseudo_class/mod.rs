mod function_compound_selector;
mod function_compound_selector_list;
mod function_identifier;
mod function_nth;
mod function_relative_selector_list;
mod function_selector;
mod function_selector_list;
mod function_value_list;
mod identifier;

use self::function_compound_selector_list::{
    is_at_pseudo_class_function_compound_selector_list,
    parse_pseudo_class_function_compound_selector_list,
};
use self::function_identifier::{
    is_at_pseudo_class_function_identifier, parse_pseudo_class_function_identifier,
};
use self::function_nth::{is_at_pseudo_class_function_nth, parse_pseudo_class_function_nth};
use self::function_relative_selector_list::{
    is_at_pseudo_class_function_relative_selector_list,
    parse_pseudo_class_function_relative_selector_list,
};
use self::function_selector::{
    is_at_pseudo_class_function_selector, parse_pseudo_class_function_selector,
};
use self::function_selector_list::{
    is_at_pseudo_class_function_selector_list, parse_pseudo_class_function_selector_list,
};
use self::function_value_list::{
    is_at_pseudo_class_function_value_list, parse_pseudo_class_function_value_list,
};
use self::identifier::parse_pseudo_class_identifier;
use crate::parser::CssParser;
use crate::syntax::is_at_identifier;
use crate::syntax::parse_error::expect_any_pseudo_class;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::Parser;
use function_compound_selector::{
    is_at_pseudo_class_function_compound_selector, parse_pseudo_class_function_compound_selector,
};

#[inline]
pub(crate) fn parse_pseudo_class_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![:]);
    parse_pseudo_class(p).or_add_diagnostic(p, expect_any_pseudo_class);

    Present(m.complete(p, CSS_PSEUDO_CLASS_SELECTOR))
}

#[inline]
fn parse_pseudo_class(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    if is_at_pseudo_class_function_identifier(p) {
        parse_pseudo_class_function_identifier(p)
    } else if is_at_pseudo_class_function_selector(p) {
        parse_pseudo_class_function_selector(p)
    } else if is_at_pseudo_class_function_selector_list(p) {
        parse_pseudo_class_function_selector_list(p)
    } else if is_at_pseudo_class_function_compound_selector(p) {
        parse_pseudo_class_function_compound_selector(p)
    } else if is_at_pseudo_class_function_compound_selector_list(p) {
        parse_pseudo_class_function_compound_selector_list(p)
    } else if is_at_pseudo_class_function_relative_selector_list(p) {
        parse_pseudo_class_function_relative_selector_list(p)
    } else if is_at_pseudo_class_function_value_list(p) {
        parse_pseudo_class_function_value_list(p)
    } else if is_at_pseudo_class_function_nth(p) {
        parse_pseudo_class_function_nth(p)
    } else {
        parse_pseudo_class_identifier(p)
    }
}
