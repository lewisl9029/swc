#![deny(clippy::all)]
#![allow(clippy::needless_update)]

pub use self::emit::*;
use self::{ctx::Ctx, list::ListFormat};
pub use std::fmt::Result;
use swc_common::Spanned;
use swc_css_ast::*;
use swc_css_codegen_macros::emitter;
use writer::CssWriter;

#[macro_use]
mod macros;
mod ctx;
mod emit;
mod list;
pub mod writer;

#[derive(Debug, Clone, Copy)]
pub struct CodegenConfig {
    pub minify: bool,
}
#[derive(Debug)]
pub struct CodeGenerator<W>
where
    W: CssWriter,
{
    wr: W,
    config: CodegenConfig,
    ctx: Ctx,
}

impl<W> CodeGenerator<W>
where
    W: CssWriter,
{
    pub fn new(wr: W, config: CodegenConfig) -> Self {
        CodeGenerator {
            wr,
            config,
            ctx: Default::default(),
        }
    }

    #[emitter]
    fn emit_stylesheet(&mut self, n: &Stylesheet) -> Result {
        self.emit_list(
            &n.rules,
            if self.config.minify {
                ListFormat::NotDelimited
            } else {
                ListFormat::NotDelimited | ListFormat::MultiLine
            },
        )?;
    }

    #[emitter]
    fn emit_rule(&mut self, n: &Rule) -> Result {
        match n {
            Rule::QualifiedRule(n) => emit!(self, n),
            Rule::AtRule(n) => emit!(self, n),
            Rule::Invalid(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_qualified_rule(&mut self, n: &QualifiedRule) -> Result {
        emit!(self, n.prelude);
        formatting_space!(self);
        emit!(self, n.block);
    }

    #[emitter]
    fn emit_at_rule(&mut self, n: &AtRule) -> Result {
        match n {
            AtRule::Charset(n) => emit!(self, n),
            AtRule::Import(n) => emit!(self, n),
            AtRule::FontFace(n) => emit!(self, n),
            AtRule::Keyframes(n) => emit!(self, n),
            AtRule::Layer(n) => emit!(self, n),
            AtRule::Media(n) => emit!(self, n),
            AtRule::Supports(n) => emit!(self, n),
            AtRule::Page(n) => emit!(self, n),
            AtRule::Namespace(n) => emit!(self, n),
            AtRule::Viewport(n) => emit!(self, n),
            AtRule::Document(n) => emit!(self, n),
            AtRule::ColorProfile(n) => emit!(self, n),
            AtRule::Unknown(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_import_supports_type(&mut self, n: &ImportSupportsType) -> Result {
        match n {
            ImportSupportsType::SupportsCondition(n) => emit!(self, n),
            ImportSupportsType::Declaration(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_charset_rule(&mut self, n: &CharsetRule) -> Result {
        punct!(self, "@");
        keyword!(self, "charset");
        // https://drafts.csswg.org/css2/#charset%E2%91%A0
        // @charset must be written literally, i.e., the 10 characters '@charset "'
        // (lowercase, no backslash escapes), followed by the encoding name, followed by
        // ";.
        space!(self);
        emit!(self, n.charset);
        semi!(self);
    }

    #[emitter]
    fn emit_import_rule(&mut self, n: &ImportRule) -> Result {
        punct!(self, "@");
        keyword!(self, "import");

        match n.href {
            ImportHref::Url(_) => {
                space!(self);
            }
            ImportHref::Str(_) => {
                formatting_space!(self);
            }
        }

        emit!(self, n.href);

        if let Some(layer_name) = &n.layer_name {
            formatting_space!(self);
            emit!(self, layer_name);

            if self.config.minify && (n.supports.is_some() || n.media.is_some()) {
                if let ImportLayerName::Ident(_) = layer_name {
                    space!(self);
                }
            }
        }

        if let Some(supports) = &n.supports {
            formatting_space!(self);
            keyword!(self, "supports");
            punct!(self, "(");
            emit!(self, supports);
            punct!(self, ")");
        }

        if let Some(media) = &n.media {
            formatting_space!(self);
            emit!(self, media);
        }

        semi!(self);
    }

    #[emitter]
    fn emit_import_href(&mut self, n: &ImportHref) -> Result {
        match n {
            ImportHref::Url(n) => emit!(self, n),
            ImportHref::Str(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_import_layer_name(&mut self, n: &ImportLayerName) -> Result {
        match n {
            ImportLayerName::Ident(n) => emit!(self, n),
            ImportLayerName::Function(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_font_face_rule(&mut self, n: &FontFaceRule) -> Result {
        punct!(self, "@");
        keyword!(self, "font-face");
        formatting_space!(self);
        emit!(self, n.block);
    }

    #[emitter]
    fn emit_keyframes_rule(&mut self, n: &KeyframesRule) -> Result {
        punct!(self, "@");
        keyword!(self, "keyframes");

        match n.name {
            KeyframesName::Str(_) => {
                formatting_space!(self);
            }
            KeyframesName::CustomIdent(_) => {
                space!(self);
            }
        }

        emit!(self, n.name);
        formatting_space!(self);
        punct!(self, "{");
        formatting_newline!(self);

        self.emit_list(
            &n.blocks,
            if self.config.minify {
                ListFormat::NotDelimited
            } else {
                ListFormat::MultiLine
            },
        )?;

        formatting_newline!(self);
        punct!(self, "}");
    }

    #[emitter]
    fn emit_keyframes_name(&mut self, n: &KeyframesName) -> Result {
        match n {
            KeyframesName::CustomIdent(n) => emit!(self, n),
            KeyframesName::Str(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_keyframe_block(&mut self, n: &KeyframeBlock) -> Result {
        self.emit_list(&n.prelude, ListFormat::CommaDelimited)?;

        formatting_space!(self);

        emit!(self, n.block);
    }

    #[emitter]
    fn emit_keyframe_selector(&mut self, n: &KeyframeSelector) -> Result {
        match n {
            KeyframeSelector::Ident(n) => emit!(self, n),
            KeyframeSelector::Percent(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_layer_name(&mut self, n: &LayerName) -> Result {
        self.emit_list(&n.name, ListFormat::DotDelimited)?;
    }

    #[emitter]
    fn emit_layer_name_list(&mut self, n: &LayerNameList) -> Result {
        self.emit_list(&n.name_list, ListFormat::CommaDelimited)?;
    }

    #[emitter]
    fn emit_layer_prelude(&mut self, n: &LayerPrelude) -> Result {
        match n {
            LayerPrelude::Name(n) => emit!(self, n),
            LayerPrelude::NameList(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_layer_rule(&mut self, n: &LayerRule) -> Result {
        punct!(self, "@");
        keyword!(self, "layer");

        if n.prelude.is_some() {
            space!(self);
            emit!(self, n.prelude);
        } else {
            formatting_space!(self);
        }

        if let Some(rules) = &n.rules {
            punct!(self, "{");
            self.emit_list(
                rules,
                if self.config.minify {
                    ListFormat::NotDelimited
                } else {
                    ListFormat::NotDelimited | ListFormat::MultiLine
                },
            )?;
            punct!(self, "}");
        } else {
            punct!(self, ";");
        }
    }

    #[emitter]
    fn emit_media_rule(&mut self, n: &MediaRule) -> Result {
        punct!(self, "@");
        keyword!(self, "media");

        if n.media.is_some() {
            let need_space = match n.media.as_ref().unwrap().queries.get(0) {
                Some(media_query)
                    if media_query.modifier.is_none() && media_query.media_type.is_none() =>
                {
                    match &media_query.condition {
                        Some(MediaConditionType::All(media_condition)) => !matches!(
                            media_condition.conditions.get(0),
                            Some(MediaConditionAllType::MediaInParens(_))
                        ),
                        _ => true,
                    }
                }
                _ => true,
            };

            if need_space {
                space!(self);
            } else {
                formatting_space!(self);
            }

            emit!(self, n.media);

            formatting_space!(self);
        } else {
            formatting_space!(self);
        }

        punct!(self, "{");
        self.emit_list(&n.rules, ListFormat::NotDelimited | ListFormat::MultiLine)?;
        punct!(self, "}");
    }

    #[emitter]
    fn emit_media_query_list(&mut self, n: &MediaQueryList) -> Result {
        self.emit_list(&n.queries, ListFormat::CommaDelimited)?;
    }

    #[emitter]
    fn emit_media_query(&mut self, n: &MediaQuery) -> Result {
        if n.modifier.is_some() {
            emit!(self, n.modifier);
            space!(self);
        }

        if n.media_type.is_some() {
            emit!(self, n.media_type);

            if n.condition.is_some() {
                space!(self);
                keyword!(self, "and");
                space!(self);
            }
        }

        if n.condition.is_some() {
            emit!(self, n.condition);
        }
    }

    #[emitter]
    fn emit_media_condition_type(&mut self, n: &MediaConditionType) -> Result {
        match n {
            MediaConditionType::All(n) => emit!(self, n),
            MediaConditionType::WithoutOr(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_media_condition(&mut self, n: &MediaCondition) -> Result {
        self.emit_list(&n.conditions, ListFormat::NotDelimited)?;
    }

    #[emitter]
    fn emit_media_condition_without_or(&mut self, n: &MediaConditionWithoutOr) -> Result {
        self.emit_list(&n.conditions, ListFormat::NotDelimited)?;
    }

    #[emitter]
    fn emit_media_condition_all_type(&mut self, n: &MediaConditionAllType) -> Result {
        match n {
            MediaConditionAllType::Not(n) => emit!(self, n),
            MediaConditionAllType::And(n) => emit!(self, n),
            MediaConditionAllType::Or(n) => emit!(self, n),
            MediaConditionAllType::MediaInParens(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_media_condition_without_or_type(&mut self, n: &MediaConditionWithoutOrType) -> Result {
        match n {
            MediaConditionWithoutOrType::Not(n) => emit!(self, n),
            MediaConditionWithoutOrType::And(n) => emit!(self, n),
            MediaConditionWithoutOrType::MediaInParens(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_media_not(&mut self, n: &MediaNot) -> Result {
        formatting_space!(self);
        keyword!(self, "not");
        space!(self);
        emit!(self, n.condition);
    }

    #[emitter]
    fn emit_media_and(&mut self, n: &MediaAnd) -> Result {
        formatting_space!(self);
        keyword!(self, "and");
        space!(self);
        emit!(self, n.condition);
    }

    #[emitter]
    fn emit_media_or(&mut self, n: &MediaOr) -> Result {
        formatting_space!(self);
        keyword!(self, "or");
        space!(self);
        emit!(self, n.condition);
    }

    #[emitter]
    fn emit_media_in_parens(&mut self, n: &MediaInParens) -> Result {
        match n {
            MediaInParens::MediaCondition(n) => {
                punct!(self, "(");
                emit!(self, n);
                punct!(self, ")");
            }
            MediaInParens::Feature(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_media_feature(&mut self, n: &MediaFeature) -> Result {
        punct!(self, "(");

        match n {
            MediaFeature::Plain(n) => emit!(self, n),
            MediaFeature::Boolean(n) => emit!(self, n),
            MediaFeature::Range(n) => emit!(self, n),
            MediaFeature::RangeInterval(n) => emit!(self, n),
        }

        punct!(self, ")");
    }

    #[emitter]
    fn emit_media_feature_name(&mut self, n: &MediaFeatureName) -> Result {
        match n {
            MediaFeatureName::Ident(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_media_feature_value(&mut self, n: &MediaFeatureValue) -> Result {
        match n {
            MediaFeatureValue::Number(n) => emit!(self, n),
            MediaFeatureValue::Dimension(n) => emit!(self, n),
            MediaFeatureValue::Ident(n) => emit!(self, n),
            MediaFeatureValue::Ratio(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_media_feature_plain(&mut self, n: &MediaFeaturePlain) -> Result {
        emit!(self, n.name);
        punct!(self, ":");
        formatting_space!(self);
        emit!(self, n.value);
    }

    #[emitter]
    fn emit_media_feature_boolean(&mut self, n: &MediaFeatureBoolean) -> Result {
        emit!(self, n.name);
    }

    #[emitter]
    fn emit_media_feature_range(&mut self, n: &MediaFeatureRange) -> Result {
        emit!(self, n.left);
        formatting_space!(self);
        self.wr.write_punct(None, n.comparison.as_str())?;
        formatting_space!(self);
        emit!(self, n.right);
    }

    #[emitter]
    fn emit_media_feature_range_interval(&mut self, n: &MediaFeatureRangeInterval) -> Result {
        emit!(self, n.left);
        formatting_space!(self);
        self.wr.write_punct(None, n.left_comparison.as_str())?;
        formatting_space!(self);
        emit!(self, n.name);
        formatting_space!(self);
        self.wr.write_punct(None, n.right_comparison.as_str())?;
        formatting_space!(self);
        emit!(self, n.right);
    }

    #[emitter]
    fn emit_supports_rule(&mut self, n: &SupportsRule) -> Result {
        punct!(self, "@");
        keyword!(self, "supports");

        match n.condition.conditions.get(0) {
            Some(SupportsConditionType::SupportsInParens(_)) => {
                formatting_space!(self);
            }
            _ => {
                space!(self);
            }
        }

        emit!(self, n.condition);
        formatting_space!(self);
        punct!(self, "{");
        self.emit_list(&n.rules, ListFormat::NotDelimited)?;
        punct!(self, "}");
    }

    #[emitter]
    fn emit_supports_condition(&mut self, n: &SupportsCondition) -> Result {
        self.emit_list(&n.conditions, ListFormat::NotDelimited)?;
    }

    #[emitter]
    fn emit_supports_condition_type(&mut self, n: &SupportsConditionType) -> Result {
        match n {
            SupportsConditionType::Not(n) => emit!(self, n),
            SupportsConditionType::And(n) => emit!(self, n),
            SupportsConditionType::Or(n) => emit!(self, n),
            SupportsConditionType::SupportsInParens(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_supports_not(&mut self, n: &SupportsNot) -> Result {
        formatting_space!(self);
        keyword!(self, "not");
        space!(self);
        emit!(self, n.condition);
    }

    #[emitter]
    fn emit_supports_and(&mut self, n: &SupportsAnd) -> Result {
        formatting_space!(self);
        keyword!(self, "and");
        space!(self);
        emit!(self, n.condition);
    }

    #[emitter]
    fn emit_support_or(&mut self, n: &SupportsOr) -> Result {
        formatting_space!(self);
        keyword!(self, "or");
        space!(self);
        emit!(self, n.condition);
    }

    #[emitter]
    fn emit_supports_in_parens(&mut self, n: &SupportsInParens) -> Result {
        match n {
            SupportsInParens::SupportsCondition(n) => {
                punct!(self, "(");
                emit!(self, n);
                punct!(self, ")");
            }
            SupportsInParens::Feature(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_supports_feature(&mut self, n: &SupportsFeature) -> Result {
        punct!(self, "(");

        match n {
            SupportsFeature::Declaration(n) => emit!(self, n),
        }

        punct!(self, ")");
    }

    #[emitter]
    fn emit_page_rule(&mut self, n: &PageRule) -> Result {
        punct!(self, "@");
        keyword!(self, "page");
        space!(self);

        self.emit_list(&n.prelude, ListFormat::CommaDelimited)?;

        emit!(self, n.block);
    }

    #[emitter]
    fn emit_page_selector(&mut self, n: &PageSelector) -> Result {
        emit!(self, n.ident);
        if let Some(pseudo) = &n.pseudo {
            punct!(self, ":");
            emit!(self, pseudo);
        }
    }
    #[emitter]
    fn emit_namespace_uri(&mut self, n: &NamespaceUri) -> Result {
        match n {
            NamespaceUri::Url(n) => emit!(self, n),
            NamespaceUri::Str(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_namespace_rule(&mut self, n: &NamespaceRule) -> Result {
        punct!(self, "@");
        keyword!(self, "namespace");

        let has_prefix = n.prefix.is_some();
        let is_uri_url = match n.uri {
            NamespaceUri::Url(_) => true,
            NamespaceUri::Str(_) => false,
        };

        if has_prefix || is_uri_url {
            space!(self);
        } else {
            formatting_space!(self);
        }

        if has_prefix {
            emit!(self, n.prefix);

            if is_uri_url {
                space!(self);
            } else {
                formatting_space!(self);
            }
        }

        emit!(self, n.uri);
        punct!(self, ";");
    }

    #[emitter]
    fn emit_viewport_rule(&mut self, n: &ViewportRule) -> Result {
        punct!(self, "@");
        keyword!(self, "viewport");
        formatting_space!(self);

        emit!(self, n.block);
    }

    #[emitter]
    fn emit_document_rule(&mut self, n: &DocumentRule) -> Result {
        punct!(self, "@");
        keyword!(self, "document");
        space!(self);

        self.emit_list(&n.selectors, ListFormat::CommaDelimited)?;

        formatting_space!(self);

        punct!(self, "{");
        self.emit_list(&n.block, ListFormat::NotDelimited)?;
        punct!(self, "}");
    }

    fn emit_list_values(&mut self, nodes: &[Value], format: ListFormat) -> Result {
        let iter = nodes.iter();

        for (idx, node) in iter.enumerate() {
            emit!(self, node);

            if idx != nodes.len() - 1 {
                let need_delim = match node {
                    Value::SimpleBlock(_)
                    | Value::Function(_)
                    | Value::Delimiter(_)
                    | Value::Str(_)
                    | Value::Url(_)
                    | Value::Percent(_) => match nodes.get(idx + 1) {
                        Some(Value::Delimiter(Delimiter {
                            value: DelimiterValue::Comma,
                            ..
                        })) => false,
                        _ => !self.config.minify,
                    },
                    _ => match nodes.get(idx + 1) {
                        Some(Value::SimpleBlock(_)) | Some(Value::Color(Color::HexColor(_))) => {
                            !self.config.minify
                        }
                        Some(Value::Delimiter(_)) => false,
                        _ => true,
                    },
                };

                if need_delim {
                    self.write_delim(format)?;
                }
            }
        }

        Ok(())
    }

    #[emitter]
    fn emit_function(&mut self, n: &Function) -> Result {
        emit!(self, n.name);
        punct!(self, "(");
        self.emit_list_values(
            &n.value,
            ListFormat::SpaceDelimited | ListFormat::SingleLine,
        )?;
        punct!(self, ")");
    }

    #[emitter]
    fn emit_value(&mut self, n: &Value) -> Result {
        match n {
            Value::Function(n) => emit!(self, n),
            Value::SimpleBlock(n) => emit!(self, n),
            Value::Dimension(n) => emit!(self, n),
            Value::Number(n) => emit!(self, n),
            Value::Percent(n) => emit!(self, n),
            Value::Ratio(n) => emit!(self, n),
            Value::Color(n) => emit!(self, n),
            Value::Ident(n) => emit!(self, n),
            Value::DashedIdent(n) => emit!(self, n),
            Value::Str(n) => emit!(self, n),
            Value::Bin(n) => emit!(self, n),
            Value::Tokens(n) => emit!(self, n),
            Value::Url(n) => emit!(self, n),
            Value::Delimiter(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_at_rule_name(&mut self, n: &AtRuleName) -> Result {
        match n {
            AtRuleName::Ident(n) => emit!(self, n),
            AtRuleName::DashedIdent(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_color_profile_name(&mut self, n: &ColorProfileName) -> Result {
        match n {
            ColorProfileName::Ident(n) => emit!(self, n),
            ColorProfileName::DashedIdent(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_color_profile_rule(&mut self, n: &ColorProfileRule) -> Result {
        punct!(self, "@");
        keyword!(self, "color-profile");
        space!(self);
        emit!(self, n.name);
        formatting_space!(self);
        punct!(self, "{");
        self.emit_list(&n.block, ListFormat::NotDelimited)?;
        punct!(self, "}");
    }

    #[emitter]
    fn emit_unknown_at_rule(&mut self, n: &UnknownAtRule) -> Result {
        punct!(self, "@");
        emit!(self, n.name);

        self.emit_list(&n.prelude, ListFormat::NotDelimited)?;

        if n.block.is_some() {
            emit!(self, n.block)
        } else {
            punct!(self, ";");
        }
    }

    #[emitter]
    fn emit_str(&mut self, n: &Str) -> Result {
        if self.config.minify {
            self.wr.write_str(Some(n.span), &n.value)?;
        } else {
            self.wr.write_raw(Some(n.span), &n.raw)?;
        }
    }

    #[emitter]
    fn emit_simple_block(&mut self, n: &SimpleBlock) -> Result {
        let ending = match n.name {
            '[' => ']',
            '(' => ')',
            '{' => '}',
            _ => {
                unreachable!();
            }
        };

        self.wr.write_raw_char(None, n.name)?;
        self.emit_list(
            &n.value,
            if ending == ']' {
                ListFormat::SpaceDelimited
            } else {
                ListFormat::NotDelimited
            },
        )?;
        self.wr.write_raw_char(None, ending)?;
    }

    #[emitter]
    fn emit_block(&mut self, n: &Block) -> Result {
        punct!(self, "{");

        self.emit_list(
            &n.value,
            if self.config.minify {
                ListFormat::SemiDelimited
            } else {
                ListFormat::SemiDelimited | ListFormat::MultiLine
            },
        )?;

        punct!(self, "}");
    }

    #[emitter]
    fn emit_declaration_block_item(&mut self, n: &DeclarationBlockItem) -> Result {
        match n {
            DeclarationBlockItem::Declaration(n) => emit!(self, n),
            DeclarationBlockItem::AtRule(n) => emit!(self, n),
            DeclarationBlockItem::Invalid(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_declaration(&mut self, n: &Declaration) -> Result {
        emit!(self, n.name);
        punct!(self, ":");

        let is_custom_property = match n.name {
            DeclarationName::DashedIdent(_) => true,
            DeclarationName::Ident(_) => false,
        };

        // https://github.com/w3c/csswg-drafts/issues/774
        // `--foo: ;` and `--foo:;` is valid, but not all browsers support it, currently
        // we print " " (whitespace) always
        if is_custom_property {
            match n.value.get(0) {
                Some(Value::Tokens(tokens)) if tokens.tokens.is_empty() => {
                    space!(self);
                }
                _ => {
                    formatting_space!(self);
                }
            };
        } else {
            formatting_space!(self);
        }

        if is_custom_property {
            self.emit_list(&n.value, ListFormat::NotDelimited)?;
        } else {
            self.emit_list_values(
                &n.value,
                ListFormat::SpaceDelimited | ListFormat::SingleLine,
            )?;
        }

        if n.important.is_some() {
            if !is_custom_property {
                formatting_space!(self);
            }

            emit!(self, n.important);
        }

        if self.ctx.semi_after_property {
            punct!(self, ";");
        }
    }

    #[emitter]
    fn emit_declaration_name(&mut self, n: &DeclarationName) -> Result {
        match n {
            DeclarationName::Ident(n) => emit!(self, n),
            DeclarationName::DashedIdent(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_important_flag(&mut self, n: &ImportantFlag) -> Result {
        punct!(self, "!");

        if self.config.minify {
            self.wr.write_raw(None, "important")?;
        } else {
            emit!(self, n.value);
        }
    }

    #[emitter]
    fn emit_ident(&mut self, n: &Ident) -> Result {
        self.wr.write_raw(Some(n.span), &n.raw)?;
    }

    #[emitter]
    fn emit_custom_ident(&mut self, n: &CustomIdent) -> Result {
        self.wr.write_raw(Some(n.span), &n.raw)?;
    }

    #[emitter]
    fn emit_dashed_ident(&mut self, n: &DashedIdent) -> Result {
        self.wr.write_raw(Some(n.span), &n.raw)?;
    }

    #[emitter]
    fn emit_percent(&mut self, n: &Percent) -> Result {
        emit!(self, n.value);
        punct!(self, "%");
    }

    #[emitter]
    fn emit_page_rule_block(&mut self, n: &PageRuleBlock) -> Result {
        punct!(self, "{");
        formatting_newline!(self);

        self.wr.increase_indent();

        let ctx = Ctx {
            semi_after_property: true,
            ..self.ctx
        };
        self.with_ctx(ctx)
            .emit_list(&n.items, ListFormat::MultiLine | ListFormat::NotDelimited)?;
        formatting_newline!(self);

        self.wr.decrease_indent();

        punct!(self, "}");
    }

    #[emitter]
    fn emit_page_rule_block_item(&mut self, n: &PageRuleBlockItem) -> Result {
        match n {
            PageRuleBlockItem::Declaration(n) => emit!(self, n),
            PageRuleBlockItem::Nested(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_dimension(&mut self, n: &Dimension) -> Result {
        emit!(self, n.value);
        emit!(self, n.unit);
    }

    #[emitter]
    fn emit_number(&mut self, n: &Number) -> Result {
        if self.config.minify {
            if n.value.is_sign_negative() && n.value == 0.0 {
                self.wr.write_raw(Some(n.span), "-0")?;
            } else {
                let mut minified = n.value.to_string();

                if minified.starts_with("0.") {
                    minified.replace_range(0..1, "");
                } else if minified.starts_with("-0.") {
                    minified.replace_range(1..2, "");
                }

                if minified.starts_with(".000") {
                    let mut cnt = 3;

                    for &v in minified.as_bytes().iter().skip(4) {
                        if v == b'0' {
                            cnt += 1;
                        } else {
                            break;
                        }
                    }

                    minified.replace_range(0..cnt + 1, "");

                    let remain_len = minified.len();

                    minified.push_str("e-");
                    minified.push_str(&(remain_len + cnt).to_string());
                } else if minified.ends_with("000") {
                    let mut cnt = 3;

                    for &v in minified.as_bytes().iter().rev().skip(3) {
                        if v == b'0' {
                            cnt += 1;
                        } else {
                            break;
                        }
                    }

                    minified.truncate(minified.len() - cnt);
                    minified.push('e');
                    minified.push_str(&cnt.to_string());
                }

                self.wr.write_raw(Some(n.span), &minified)?;
            }
        } else {
            self.wr.write_raw(Some(n.span), &n.raw)?;
        }
    }

    #[emitter]
    fn emit_ration(&mut self, n: &Ratio) -> Result {
        emit!(self, n.left);

        if let Some(right) = &n.right {
            punct!(self, "/");
            emit!(self, right);
        }
    }

    #[emitter]
    fn emit_color(&mut self, n: &Color) -> Result {
        match n {
            Color::HexColor(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_hex_color(&mut self, n: &HexColor) -> Result {
        punct!(self, "#");

        if self.config.minify {
            let minified = minify_hex_color(&n.value);

            self.wr.write_raw(Some(n.span), &minified)?;
        } else {
            self.wr.write_raw(Some(n.span), &n.raw)?;
        }
    }

    #[emitter]
    fn emit_bin_value(&mut self, n: &BinValue) -> Result {
        emit!(self, n.left);

        let need_space = matches!(n.op, BinOp::Add | BinOp::Sub);

        if need_space {
            space!(self);
        } else {
            formatting_space!(self);
        }

        punct!(self, n.op.as_str());

        if need_space {
            space!(self);
        } else {
            formatting_space!(self);
        }

        emit!(self, n.right);
    }

    #[emitter]
    fn emit_delimiter(&mut self, n: &Delimiter) -> Result {
        punct!(self, n.value.as_str());
    }

    #[emitter]
    fn emit_tokens(&mut self, n: &Tokens) -> Result {
        for TokenAndSpan { span, token } in &n.tokens {
            let span = *span;
            match token {
                Token::AtKeyword { raw, .. } => {
                    punct!(self, span, "@");
                    self.wr.write_raw(Some(n.span), raw)?;
                }
                Token::Delim { value } => {
                    self.wr.write_raw_char(Some(n.span), *value)?;
                }
                Token::LParen => {
                    punct!(self, span, "(");
                }
                Token::RParen => {
                    punct!(self, span, ")");
                }
                Token::LBracket => {
                    punct!(self, span, "[");
                }
                Token::RBracket => {
                    punct!(self, span, "]");
                }
                Token::Num { raw, .. } => {
                    self.wr.write_raw(Some(span), raw)?;
                }
                Token::Percent { raw, .. } => {
                    self.wr.write_raw(Some(span), raw)?;
                    punct!(self, "%");
                }
                Token::Dimension {
                    raw_value,
                    raw_unit,
                    ..
                } => {
                    self.wr.write_raw(Some(span), raw_value)?;
                    self.wr.write_raw(Some(span), raw_unit)?;
                }
                Token::Ident { raw, .. } => {
                    self.wr.write_raw(Some(n.span), raw)?;
                }
                Token::Function { raw, .. } => {
                    self.wr.write_raw(Some(n.span), raw)?;
                    punct!(self, "(");
                }
                Token::BadStr { raw, .. } => {
                    self.wr.write_raw(Some(span), raw)?;
                }
                Token::Str { raw, .. } => {
                    self.wr.write_raw(Some(span), raw)?;
                }
                Token::Url {
                    raw_name,
                    raw_value,
                    before,
                    after,
                    ..
                } => {
                    self.wr.write_raw(None, raw_name)?;
                    punct!(self, "(");
                    self.wr.write_raw(None, before)?;
                    self.wr.write_raw(None, raw_value)?;
                    self.wr.write_raw(None, after)?;
                    punct!(self, ")");
                }
                Token::BadUrl {
                    raw_name,
                    raw_value,
                    ..
                } => {
                    self.wr.write_raw(Some(span), raw_name)?;
                    punct!(self, "(");
                    self.wr.write_raw(None, raw_value)?;
                    punct!(self, ")");
                }
                Token::Comma => {
                    punct!(self, span, ",");
                }
                Token::Semi => {
                    punct!(self, span, ";");
                }
                Token::LBrace => {
                    punct!(self, span, "{");
                }
                Token::RBrace => {
                    punct!(self, span, "}");
                }
                Token::Colon => {
                    punct!(self, span, ":");
                }
                Token::Hash { raw, .. } => {
                    punct!(self, "#");
                    self.wr.write_raw(Some(span), raw)?;
                }
                Token::WhiteSpace { value, .. } => {
                    self.wr.write_raw(None, value)?;
                }
                Token::CDC => {
                    punct!(self, span, "-->");
                }
                Token::CDO => {
                    punct!(self, span, "<!--");
                }
            }
        }
    }

    #[emitter]
    fn emit_url(&mut self, n: &Url) -> Result {
        emit!(self, n.name);
        punct!(self, "(");

        if let Some(value) = &n.value {
            emit!(self, value);
        }

        if let Some(modifiers) = &n.modifiers {
            if !modifiers.is_empty() {
                formatting_space!(self);
                self.emit_list(modifiers, ListFormat::SpaceDelimited)?;
            }
        }

        punct!(self, ")");
    }

    #[emitter]
    fn emit_url_value(&mut self, n: &UrlValue) -> Result {
        match n {
            UrlValue::Raw(n) => emit!(self, n),
            UrlValue::Str(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_url_value_raw(&mut self, n: &UrlValueRaw) -> Result {
        if !self.config.minify {
            self.wr.write_raw(Some(n.span), &n.before)?;
        }

        if self.config.minify {
            self.wr.write_raw(Some(n.span), &n.value)?;
        } else {
            self.wr.write_raw(Some(n.span), &n.raw)?;
        }

        if !self.config.minify {
            self.wr.write_raw(Some(n.span), &n.after)?;
        }
    }

    #[emitter]
    fn emit_url_modifier(&mut self, n: &UrlModifier) -> Result {
        match n {
            UrlModifier::Ident(n) => emit!(self, n),
            UrlModifier::Function(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_nested_page_rule(&mut self, n: &NestedPageRule) -> Result {
        emit!(self, n.prelude);
        emit!(self, n.block);
    }

    #[emitter]
    fn emit_selector_list(&mut self, n: &SelectorList) -> Result {
        self.emit_list(&n.children, ListFormat::CommaDelimited)?;
    }

    #[emitter]
    fn emit_complex_selector(&mut self, n: &ComplexSelector) -> Result {
        let mut need_space = false;
        for (idx, node) in n.children.iter().enumerate() {
            if let ComplexSelectorChildren::Combinator(..) = node {
                need_space = false;
            }

            if idx != 0 && need_space {
                need_space = false;

                self.wr.write_space()?;
            }

            if let ComplexSelectorChildren::CompoundSelector(..) = node {
                need_space = true;
            }

            emit!(self, node)
        }
    }

    #[emitter]
    fn emit_complex_selector_children(&mut self, n: &ComplexSelectorChildren) -> Result {
        match n {
            ComplexSelectorChildren::CompoundSelector(n) => emit!(self, n),
            ComplexSelectorChildren::Combinator(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_compound_selector(&mut self, n: &CompoundSelector) -> Result {
        emit!(&mut *self.with_ctx(self.ctx), n.nesting_selector);
        emit!(&mut *self.with_ctx(self.ctx), n.type_selector);

        self.emit_list(&n.subclass_selectors, ListFormat::NotDelimited)?;
    }

    #[emitter]
    fn emit_combinator(&mut self, n: &Combinator) -> Result {
        self.wr.write_punct(None, n.value.as_str())?;
    }

    #[emitter]
    fn emit_nesting_selector(&mut self, _: &NestingSelector) -> Result {
        punct!(self, "&");
    }

    #[emitter]
    fn emit_subclass_selector(&mut self, n: &SubclassSelector) -> Result {
        match n {
            SubclassSelector::Id(n) => emit!(self, n),
            SubclassSelector::Class(n) => emit!(self, n),
            SubclassSelector::Attr(n) => emit!(self, n),
            SubclassSelector::PseudoClass(n) => emit!(self, n),
            SubclassSelector::PseudoElement(n) => emit!(self, n),
            SubclassSelector::At(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_type_selector(&mut self, n: &TypeSelector) -> Result {
        if let Some(prefix) = &n.prefix {
            emit!(self, prefix);
            punct!(self, "|");
        }

        emit!(self, n.name);
    }

    #[emitter]
    fn emit_id_selector(&mut self, n: &IdSelector) -> Result {
        punct!(self, "#");
        let ctx = Ctx { ..self.ctx };
        emit!(&mut *self.with_ctx(ctx), n.text);
    }

    #[emitter]
    fn emit_class_selector(&mut self, n: &ClassSelector) -> Result {
        punct!(self, ".");
        let ctx = Ctx { ..self.ctx };
        emit!(&mut *self.with_ctx(ctx), n.text);
    }

    #[emitter]
    fn emit_attr_selector_value(&mut self, n: &AttrSelectorValue) -> Result {
        match n {
            AttrSelectorValue::Str(n) => emit!(self, n),
            AttrSelectorValue::Ident(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_attr_selector(&mut self, n: &AttrSelector) -> Result {
        punct!(self, "[");

        if let Some(prefix) = &n.prefix {
            emit!(self, prefix);
            punct!(self, "|");
        }

        emit!(self, n.name);

        if let Some(matcher) = n.matcher {
            self.wr.write_punct(None, matcher.as_str())?;
        }

        emit!(self, n.value);

        if let Some(m) = &n.modifier {
            match n.value {
                Some(AttrSelectorValue::Str(_)) => {
                    formatting_space!(self);
                }
                Some(AttrSelectorValue::Ident(_)) => {
                    space!(self);
                }
                _ => {}
            }

            self.wr.write_raw_char(None, *m)?;
        }

        punct!(self, "]");
    }

    #[emitter]
    fn emit_nth(&mut self, n: &Nth) -> Result {
        emit!(self, n.nth);

        if n.selector_list.is_some() {
            emit!(self, n.selector_list);
        }
    }

    #[emitter]
    fn emit_an_plus_b(&mut self, n: &AnPlusB) -> Result {
        if let Some(a_raw) = &n.a_raw {
            self.wr.write_raw(Some(n.span), a_raw)?;
            punct!(self, "n");
        }

        if let Some(b_raw) = &n.b_raw {
            self.wr.write_raw(Some(n.span), b_raw)?;
        }
    }

    #[emitter]
    fn emit_nth_value(&mut self, n: &NthValue) -> Result {
        match n {
            NthValue::AnPlusB(n) => emit!(self, n),
            NthValue::Ident(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_pseudo_selector_children(&mut self, n: &PseudoSelectorChildren) -> Result {
        match n {
            PseudoSelectorChildren::Nth(n) => emit!(self, n),
            PseudoSelectorChildren::Tokens(n) => emit!(self, n),
        }
    }

    #[emitter]
    fn emit_pseudo_class_selector(&mut self, n: &PseudoClassSelector) -> Result {
        punct!(self, ":");

        emit!(self, n.name);

        if n.children.is_some() {
            punct!(self, "(");
            emit!(self, n.children);
            punct!(self, ")");
        }
    }

    #[emitter]
    fn emit_pseudo_element_selector(&mut self, n: &PseudoElementSelector) -> Result {
        punct!(self, ":");
        punct!(self, ":");

        emit!(self, n.name);

        if n.children.is_some() {
            punct!(self, "(");
            emit!(self, n.children);
            punct!(self, ")");
        }
    }

    #[emitter]
    fn emit_at_selector(&mut self, n: &AtSelector) -> Result {
        punct!(self, "@");
        emit!(self, n.text);
    }

    fn emit_list<N>(&mut self, nodes: &[N], format: ListFormat) -> Result
    where
        Self: Emit<N>,
        N: Spanned,
    {
        for (idx, node) in nodes.iter().enumerate() {
            if idx != 0 {
                self.write_delim(format)?;

                if format & ListFormat::LinesMask == ListFormat::MultiLine {
                    formatting_newline!(self);
                }
            }

            emit!(self, node)
        }

        Ok(())
    }

    fn write_delim(&mut self, f: ListFormat) -> Result {
        match f & ListFormat::DelimitersMask {
            ListFormat::None => {}
            ListFormat::CommaDelimited => {
                punct!(self, ",");
                formatting_space!(self);
            }
            ListFormat::SpaceDelimited => {
                space!(self)
            }
            ListFormat::SemiDelimited => {
                punct!(self, ";")
            }
            ListFormat::DotDelimited => {
                punct!(self, ".");
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}

fn minify_hex_color(value: &str) -> String {
    let length = value.len();

    if length == 6 || length == 8 {
        let chars = value.as_bytes();

        if chars[0] == chars[1] && chars[2] == chars[3] && chars[4] == chars[5] {
            // 6 -> 3 or 8 -> 3
            if length == 6 || chars[6] == b'f' && chars[7] == b'f' {
                let mut minified = String::new();

                minified.push((chars[0] as char).to_ascii_lowercase());
                minified.push((chars[2] as char).to_ascii_lowercase());
                minified.push((chars[4] as char).to_ascii_lowercase());

                return minified;
            }
            // 8 -> 4
            else if length == 8 && chars[6] == chars[7] {
                let mut minified = String::new();

                minified.push((chars[0] as char).to_ascii_lowercase());
                minified.push((chars[2] as char).to_ascii_lowercase());
                minified.push((chars[4] as char).to_ascii_lowercase());
                minified.push((chars[6] as char).to_ascii_lowercase());

                return minified;
            }
        }
    }

    value.to_ascii_lowercase()
}
