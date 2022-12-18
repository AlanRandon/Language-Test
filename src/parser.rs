use crate::ast::Literal;

peg::parser! {
    pub grammar grammar() for str {
        /// Comments begining "//" and taking up a single line
        rule line_comment() = "//" (!"\n" [_])* ("\n" / ![_])

        /// Multiline comments beginning "/*" and ending "*/"
        rule inline_comment() = "/*" (!"*/" [_])* "*/"

        rule whitespace() = " " / "\n" / "\t" / "\r\n"

        /// Optional whitespace and comments
        rule _() = quiet!{ (whitespace() / inline_comment() / line_comment())* }

    }
}
