use crate::errors;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Common
    Equal,
    Comma,
    LineBreak,

    // SectionMetaInfoElement
    SectionMetaInfoStart, // @
    SectionMetaInfoKey(String),
    SectionMetaInfoValue(String),

    // CodeBlockElement
    CodeBlockSeparator, // |
    Code(String),

    // MetaInfoElement
    MetaInfoStart, //(
    MetaInfoKey(String),
    MetaInfoValue(String),
    MetaInfoEnd, //)
}

#[derive(Debug, PartialEq, Clone)]
enum ValueToken {
    SectionMetaInfoKey,
    SectionMetaInfoValue,
    MetaInfoKey,
    MetaInfoValue,
    Code,
}

pub fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '@' => tokens.push(Token::SectionMetaInfoStart),
            '(' => tokens.push(Token::MetaInfoStart),
            ')' => tokens.push(Token::MetaInfoEnd),
            '|' => tokens.push(Token::CodeBlockSeparator),
            '=' => tokens.push(Token::Equal),
            ',' => tokens.push(Token::Comma),
            ' ' | '　' | '\t' => {}
            '\n' | '\r' => match tokens.last() {
                Some(Token::SectionMetaInfoKey(_)) => {
                    return Err(
                        errors::SECTION_META_INFO_KEY_SHOULD_NOT_CONTAINS_LINE_BREAK.to_string()
                    );
                }
                Some(Token::MetaInfoKey(_)) => {
                    return Err(errors::META_INFO_KEY_SHOULD_NOT_CONTAINS_LINE_BREAK.to_string());
                }
                Some(Token::MetaInfoValue(_)) => {
                    return Err(errors::META_INFO_VALUE_SHOULD_NOT_CONTAINS_LINE_BREAK.to_string());
                }
                Some(Token::Comma) => {
                    return Err(errors::CODE_BLOCK_SHOULD_NOT_CONTAINS_LINE_BREAK.to_string());
                }
                _ => tokens.push(Token::LineBreak),
            },
            non_functional_char => {
                let mut token = String::new();
                token.push(non_functional_char);
                let second_token_from_last = if tokens.len() >= 2 {
                    tokens.get(tokens.len() - 2)
                } else {
                    None
                };

                // get token type
                let get_token_type_result = match tokens.last() {
                    Some(Token::SectionMetaInfoStart) => Ok(Some(ValueToken::SectionMetaInfoKey)),
                    Some(Token::MetaInfoStart) => Ok(Some(ValueToken::MetaInfoKey)),
                    Some(Token::Equal) => match second_token_from_last {
                        Some(Token::SectionMetaInfoKey(_)) => {
                            Ok(Some(ValueToken::SectionMetaInfoValue))
                        }
                        Some(Token::MetaInfoKey(_)) => Ok(Some(ValueToken::MetaInfoValue)),
                        _ => {
                            println!("111 {:?}", second_token_from_last);
                            println!("222 {:?}", tokens);
                            Err(format!(
                                "Error: Invalid token before Token::Equal: {:?}",
                                second_token_from_last
                            ))
                        }
                    },
                    _ => Ok(Some(ValueToken::Code)),
                };

                // if error, return
                let token_type = if let Ok(token_type) = get_token_type_result {
                    token_type
                } else {
                    return Err(get_token_type_result.unwrap_err());
                };

                // get token
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '\n'
                        || next_ch == '\r'
                        || next_ch == '@'
                        || next_ch == '('
                        || next_ch == ')'
                        || next_ch == '|'
                        || next_ch == '='
                        || next_ch == ','
                        || next_ch == '\t'
                    {
                        break;
                    }

                    token.push(next_ch);
                    chars.next();
                }

                // push token
                match token_type {
                    Some(ValueToken::SectionMetaInfoKey) => {
                        tokens.push(Token::SectionMetaInfoKey(token))
                    }
                    Some(ValueToken::SectionMetaInfoValue) => {
                        tokens.push(Token::SectionMetaInfoValue(token))
                    }
                    Some(ValueToken::MetaInfoKey) => tokens.push(Token::MetaInfoKey(token)),
                    Some(ValueToken::MetaInfoValue) => tokens.push(Token::MetaInfoValue(token)),
                    Some(ValueToken::Code) => tokens.push(Token::Code(token)),
                    None => {
                        return Err(format!(
                            "Error: Invalid token type: {:?}",
                            token_type.unwrap()
                        ))
                    }
                }
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod success {
        use super::*;

        #[test]
        fn section_meta_info() {
            let input = "@section=A";
            let expected = vec![
                Token::SectionMetaInfoStart,
                Token::SectionMetaInfoKey("section".to_string()),
                Token::Equal,
                Token::SectionMetaInfoValue("A".to_string()),
            ];

            let lex_result = lexer(input);
            println!("111 {:?}", lex_result);
            assert!(lex_result.is_ok());
            let tokens = lex_result.unwrap();
            assert_eq!(tokens, expected);
        }

        #[test]
        fn multiple_section_meta_info() {
            let input = "
                @section=A
                @sample=aaa
                ";
            let expected = vec![
                Token::LineBreak,
                Token::SectionMetaInfoStart,
                Token::SectionMetaInfoKey("section".to_string()),
                Token::Equal,
                Token::SectionMetaInfoValue("A".to_string()),
                Token::LineBreak,
                Token::SectionMetaInfoStart,
                Token::SectionMetaInfoKey("sample".to_string()),
                Token::Equal,
                Token::SectionMetaInfoValue("aaa".to_string()),
                Token::LineBreak,
            ];

            let lex_result = lexer(input);
            assert!(lex_result.is_ok());
            let tokens = lex_result.unwrap();
            assert_eq!(tokens, expected);
        }

        #[test]
        fn code_block() {
            let input = "
                |C|F|Fm|C|
                ";

            let expected = vec![
                Token::LineBreak,
                Token::CodeBlockSeparator,
                Token::Code("C".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("F".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("Fm".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("C".to_string()),
                Token::CodeBlockSeparator,
                Token::LineBreak,
            ];

            let lex_result = lexer(input);
            assert!(lex_result.is_ok());
            let tokens = lex_result.unwrap();
            assert_eq!(tokens, expected);
        }

        #[test]
        fn code_block_with_multiple_meta_info() {
            let input = "
                |(key=C)(sample=aaa)C|F|(key=Eb)Fm,(sample=bbb)Bb|C|
                ";

            let expected = vec![
                Token::LineBreak,
                Token::CodeBlockSeparator,
                Token::MetaInfoStart,
                Token::MetaInfoKey("key".to_string()),
                Token::Equal,
                Token::MetaInfoValue("C".to_string()),
                Token::MetaInfoEnd,
                Token::MetaInfoStart,
                Token::MetaInfoKey("sample".to_string()),
                Token::Equal,
                Token::MetaInfoValue("aaa".to_string()),
                Token::MetaInfoEnd,
                Token::Code("C".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("F".to_string()),
                Token::CodeBlockSeparator,
                Token::MetaInfoStart,
                Token::MetaInfoKey("key".to_string()),
                Token::Equal,
                Token::MetaInfoValue("Eb".to_string()),
                Token::MetaInfoEnd,
                Token::Code("Fm".to_string()),
                Token::Comma,
                Token::MetaInfoStart,
                Token::MetaInfoKey("sample".to_string()),
                Token::Equal,
                Token::MetaInfoValue("bbb".to_string()),
                Token::MetaInfoEnd,
                Token::Code("Bb".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("C".to_string()),
                Token::CodeBlockSeparator,
                Token::LineBreak,
            ];

            let lex_result = lexer(input);
            println!("222 {:?}", lex_result);
            assert!(lex_result.is_ok());
            let tokens = lex_result.unwrap();
            assert_eq!(tokens, expected);
        }

        #[test]
        fn complicated() {
            let input = "
                @section=A
                @sample=aaa
                |C|C7|F|Fm7|
                |C|C7|F|Fm7|

                @section=B
                |(key=F)Gm|Gm|F|F|
                |Gm|Gm|F|F|
                ";

            let expected = vec![
                Token::LineBreak,
                // @section=A
                Token::SectionMetaInfoStart,
                Token::SectionMetaInfoKey("section".to_string()),
                Token::Equal,
                Token::SectionMetaInfoValue("A".to_string()),
                Token::LineBreak,
                // @sample=aaa
                Token::SectionMetaInfoStart,
                Token::SectionMetaInfoKey("sample".to_string()),
                Token::Equal,
                Token::SectionMetaInfoValue("aaa".to_string()),
                Token::LineBreak,
                // |C|C7|F|Fm7|
                Token::CodeBlockSeparator,
                Token::Code("C".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("C7".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("F".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("Fm7".to_string()),
                Token::CodeBlockSeparator,
                Token::LineBreak,
                // |C|C7|F|Fm7|
                Token::CodeBlockSeparator,
                Token::Code("C".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("C7".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("F".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("Fm7".to_string()),
                Token::CodeBlockSeparator,
                Token::LineBreak,
                Token::LineBreak,
                // @section=B
                Token::SectionMetaInfoStart,
                Token::SectionMetaInfoKey("section".to_string()),
                Token::Equal,
                Token::SectionMetaInfoValue("B".to_string()),
                Token::LineBreak,
                // |(key=F)Gm|Gm|F|F|
                Token::CodeBlockSeparator,
                Token::MetaInfoStart,
                Token::MetaInfoKey("key".to_string()),
                Token::Equal,
                Token::MetaInfoValue("F".to_string()),
                Token::MetaInfoEnd,
                Token::Code("Gm".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("Gm".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("F".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("F".to_string()),
                Token::CodeBlockSeparator,
                Token::LineBreak,
                // |Gm|Gm|F|F|
                Token::CodeBlockSeparator,
                Token::Code("Gm".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("Gm".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("F".to_string()),
                Token::CodeBlockSeparator,
                Token::Code("F".to_string()),
                Token::CodeBlockSeparator,
                Token::LineBreak,
            ];

            let lex_result = lexer(input);

            assert!(lex_result.is_ok());
            let tokens = lex_result.unwrap();
            assert_eq!(tokens, expected);
        }
    }

    #[cfg(test)]
    mod failed {
        use super::*;

        #[test]
        fn section_meta_info_key_should_not_contains_line_break() {
            let input = "
            @sect
            ion=A
            ";

            let lex_result = lexer(input);
            assert!(lex_result.is_err());
            assert_eq!(
                lex_result.unwrap_err(),
                errors::SECTION_META_INFO_KEY_SHOULD_NOT_CONTAINS_LINE_BREAK
            );
        }

        #[test]
        fn meta_info_key_should_not_contains_line_break() {
            let input = "
                |(aaaa
                    aaaa=bbbb
                )C|
                ";

            let lex_result = lexer(input);
            assert!(lex_result.is_err());
            assert_eq!(
                lex_result.unwrap_err(),
                errors::META_INFO_KEY_SHOULD_NOT_CONTAINS_LINE_BREAK
            );
        }

        #[test]
        fn meta_info_value_should_not_contains_line_break() {
            let input = "
                |(aaaa=bbbb
                    bbbb)C|
                ";

            let lex_result = lexer(input);
            assert!(lex_result.is_err());
            assert_eq!(
                lex_result.unwrap_err(),
                errors::META_INFO_VALUE_SHOULD_NOT_CONTAINS_LINE_BREAK
            );
        }

        #[test]
        fn code_block_should_not_contains_line_break() {
            let input = "
                |C,
                C7|F|Fm7|
                ";

            let lex_result = lexer(input);
            println!("222 {:?}", lex_result);
            assert!(lex_result.is_err());
            assert_eq!(
                lex_result.unwrap_err(),
                errors::CODE_BLOCK_SHOULD_NOT_CONTAINS_LINE_BREAK
            );
        }
    }
}
