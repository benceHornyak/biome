mod charset;
mod color_profile;
mod container;
mod counter_style;
mod feature;
mod font_face;
mod font_palette_values;
mod keyframes;
mod media;
mod parse_error;

use crate::parser::CssParser;
use crate::syntax::at_rule::charset::{is_at_charset_at_rule, parse_charset_at_rule};
use crate::syntax::at_rule::color_profile::{
    is_color_profile_at_rule, parse_color_profile_at_rule,
};
use crate::syntax::at_rule::container::{is_at_container_at_rule, parse_container_at_rule};
use crate::syntax::at_rule::counter_style::{
    is_at_counter_style_at_rule, parse_counter_style_at_rule,
};
use crate::syntax::at_rule::font_face::{is_at_font_face_at_rule, parse_font_face_at_rule};
use crate::syntax::at_rule::font_palette_values::{
    is_at_font_palette_values_at_rule, parse_font_palette_values_at_rule,
};
use crate::syntax::at_rule::keyframes::{is_at_keyframes_at_rule, parse_keyframes_at_rule};
use crate::syntax::at_rule::media::{is_at_media_at_rule, parse_media_at_rule};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

#[inline]
pub(crate) fn at_at_rule(p: &mut CssParser) -> bool {
    p.at(T![@])
}

#[inline]
pub(crate) fn parse_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !at_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![@]);

    let kind = if parse_any_at_rule(p).is_present() {
        CSS_AT_RULE
    } else {
        CSS_BOGUS_RULE
    };

    Present(m.complete(p, kind))
}

#[inline]
pub(crate) fn parse_any_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if is_at_charset_at_rule(p) {
        parse_charset_at_rule(p)
    } else if is_color_profile_at_rule(p) {
        parse_color_profile_at_rule(p)
    } else if is_at_counter_style_at_rule(p) {
        parse_counter_style_at_rule(p)
    } else if is_at_container_at_rule(p) {
        parse_container_at_rule(p)
    } else if is_at_font_face_at_rule(p) {
        parse_font_face_at_rule(p)
    } else if is_at_font_palette_values_at_rule(p) {
        parse_font_palette_values_at_rule(p)
    } else if is_at_media_at_rule(p) {
        parse_media_at_rule(p)
    } else if is_at_keyframes_at_rule(p) {
        parse_keyframes_at_rule(p)
    } else {
        Absent
    }
}
