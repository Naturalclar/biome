use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsInitializerClause;
use rome_js_syntax::JsInitializerClauseFields;

impl FormatNodeFields<JsInitializerClause> for FormatNodeRule<JsInitializerClause> {
    fn format_fields(
        node: &JsInitializerClause,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsInitializerClauseFields {
            eq_token,
            expression,
        } = node.as_fields();

        formatted![
            formatter,
            [
                hard_group_elements(formatted![formatter, [eq_token.format()]]?),
                space_token(),
                expression.format()
            ]
        ]
    }
}
