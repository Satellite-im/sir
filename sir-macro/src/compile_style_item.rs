use litrs::StringLit;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use rsass::output::Format;

enum CompileMacroError {
    BodyMustBeString,
    ExpectedOneArgument,
}

impl CompileMacroError {
    fn to_compile_error(self) -> proc_macro2::TokenStream {
        match self {
            CompileMacroError::BodyMustBeString => {
                quote! {
                    compile_error!("Must provide CSS as a string literal")
                }
            }
            CompileMacroError::ExpectedOneArgument => {
                quote! {
                    compile_error!("Expected exactly 1 argument")
                }
            }
        }
    }
}

struct CompileMacroInput {
    body: String,
}

pub fn inner_compile_style_item(class: String, item: TokenStream) -> TokenStream {
    let input = match parse_input(item) {
        Ok(input) => input,
        Err(error) => {
            return error.to_compile_error();
        }
    };

    let body = input.body;

    let css = compile_item(&class, &body);

    quote_output_tuple(&class, &css)
}

pub fn inner_compile_global_style(item: TokenStream) -> TokenStream {
    let input = match parse_input(item) {
        Ok(input) => input,
        Err(error) => {
            return error.to_compile_error();
        }
    };

    let body = input.body;

    let css = compile_scss(&body);

    quote_string_literal(&css)
}

fn quote_output_tuple(class: &str, css: &str) -> TokenStream {
    let class_literal = proc_macro2::Literal::string(&class);
    let css_literal = proc_macro2::Literal::string(&css);

    quote!((#class_literal, #css_literal))
}

fn quote_string_literal(css: &str) -> TokenStream {
    let css_literal = proc_macro2::Literal::string(&css);
    quote!(#css_literal)
}

fn parse_input(input: TokenStream) -> Result<CompileMacroInput, CompileMacroError> {
    let tokens: Vec<_> = input.into_iter().collect();
    if tokens.len() != 1 {
        return Err(CompileMacroError::ExpectedOneArgument);
    }

    let string_literal = match &tokens[0] {
        // edge case: sometimes (when called from macro_rules definition), our literal gets wrapped in a group.
        // we need to "unwrap" it first.
        TokenTree::Group(group) => {
            let tokens: Vec<_> = group.stream().into_iter().collect();

            if tokens.len() != 1 {
                return Err(CompileMacroError::ExpectedOneArgument);
            }

            StringLit::try_from(&tokens[0])
        }
        other => StringLit::try_from(other),
    }
    .map_err(|_| CompileMacroError::BodyMustBeString)?;

    let body = string_literal.into_value().into_owned();

    Ok(CompileMacroInput { body })
}

fn compile_item(class: &str, body: &str) -> String {
    let scss = format!(".{class} {{ \n {body} \n }}");
    compile_scss(&scss)
}

fn compile_scss(scss: &str) -> String {
    let css =
        rsass::compile_scss(scss.as_bytes(), Format::default()).expect("Could not compile scss");
    String::from_utf8(css).expect("Could not convert css to string")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens_equal(a: TokenStream, b: TokenStream) {
        assert_eq!(a.to_string(), b.to_string());
    }

    #[test]
    fn empty() {
        let input = quote! {""};
        let output = inner_compile_style_item("tmp".to_string(), input);
        let expected = quote! {("tmp", "")};
        assert_tokens_equal(output, expected);
    }

    #[test]
    fn basic_rule() {
        let input = quote! {"color: red;"};
        let output = inner_compile_style_item("santa-nose".to_string(), input);
        let expected = quote! {("santa-nose", ".santa-nose {\n  color: red;\n}\n")};
        assert_tokens_equal(output, expected);
    }

    #[test]
    fn raw_string() {
        let input = quote! {r#"color: red;"#};
        let output = inner_compile_style_item("santa-nose".to_string(), input);
        let expected = quote! {("santa-nose", ".santa-nose {\n  color: red;\n}\n")};
        assert_tokens_equal(output, expected);
    }

    #[test]
    fn nesting() {
        let input = quote! {"
            color: red;
            &:hover {
                color: blue;
            }
        "};
        let output = inner_compile_style_item("santa-nose".to_string(), input);
        let expected = quote! {("santa-nose", ".santa-nose {\n  color: red;\n}\n.santa-nose:hover {\n  color: blue;\n}\n")};
        assert_tokens_equal(output, expected);
    }
}
