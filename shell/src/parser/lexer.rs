use std::str::CharIndices;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SpannedToken<'a> {
    pub token: Token<'a>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token<'a> {
    Word(&'a str),
    String(&'a str),
    Pipe,
    RedirectOutput,
    RedirectAppend,
    RedirectInput,
    SemiColon,
    And,
    Or,
}



pub struct TokenLexer<'a> {
    input: std::iter::Peekable<CharIndices<'a>>,
    string: &'a str,
}

impl<'a> TokenLexer<'a> {
    pub fn new(input: &'a str) -> TokenLexer<'a> {
        TokenLexer {
            input: input.char_indices().peekable(),
            string: input,
        }
    }
}


impl<'a> Iterator for TokenLexer<'a> {
    type Item = SpannedToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut end = 0;
        while let Some((start, c)) = self.input.next() {
            match c {
                '"' => {
                    while let Some(&(i, c)) = self.input.peek() {
                        match c {
                            '"' => {
                                end = i;
                                self.input.next();
                                return Some(SpannedToken {
                                    token: Token::String(&self.string[start + 1..end]),
                                    start,
                                    end,
                                });
                            }
                            _ => {
                                self.input.next();
                            }
                        }
                    }
                }
                '\'' => {
                    while let Some(&(i, c)) = self.input.peek() {
                        match c {
                            '\'' => {
                                end = i;
                                self.input.next();
                                return Some(SpannedToken {
                                    token: Token::String(&self.string[start + 1..end]),
                                    start,
                                    end,
                                });
                            }
                            _ => {
                                self.input.next();
                            }
                        }
                    }
                }
                '%' => {
                    let Some(&(_, c)) = self.input.peek() else {
                        return None;
                    };
                    let end = match c {
                        '(' => ')',
                        '{' => '}',
                        '[' => ']',
                        '<' => '>',
                        c if c.is_alphanumeric() => return None,
                        c => c,
                    };
                    self.input.next();

                    while let Some(&(i, c)) = self.input.peek() {
                        match c {
                            c if c == end => {
                                self.input.next();
                                return Some(SpannedToken {
                                    token: Token::String(&self.string[start + 2..=i - 1]),
                                    start,
                                    end: i,
                                });
                            }
                            _ => {
                                self.input.next();
                            }
                        }
                    }
                }
                '|' => {
                    if let Some(&(i, '|')) = self.input.peek() {
                        self.input.next();
                        return Some(SpannedToken {
                            token: Token::Or,
                            start,
                            end: i,
                        });
                    } else {
                        return Some(SpannedToken {
                            token: Token::Pipe,
                            start,
                            end,
                        });
                    }
                }
                '>' => {
                    if let Some(&(i, '>')) = self.input.peek() {
                        self.input.next();
                        return Some(SpannedToken {
                            token: Token::RedirectAppend,
                            start,
                            end: i,
                        });
                    } else {
                        return Some(SpannedToken {
                            token: Token::RedirectOutput,
                            start,
                            end,
                        });
                    }
                }
                '<' => {
                    return Some(SpannedToken {
                        token: Token::RedirectInput,
                        start,
                        end,
                    });
                }
                ';' => {
                    return Some(SpannedToken {
                        token: Token::SemiColon,
                        start,
                        end,
                    });
                }
                '&' => {
                    if let Some(&(i, c)) = self.input.peek() {
                        if c == '&' {
                            self.input.next();
                            return Some(SpannedToken {
                                token: Token::And,
                                start,
                                end: i,
                            });
                        }
                    } else {
                        return None;
                    }
                }
                c if c.is_whitespace() => {}
                _ => {
                    end = start;
                    while let Some(&(i, c)) = self.input.peek() {
                        if !c.is_whitespace() && c != '>' && c != '<'  {
                            end = i;
                            self.input.next();
                        } else {
                            break;
                        }
                    }

                    return Some(SpannedToken {
                        token: Token::Word(&self.string[start..=end]),
                        start,
                        end,
                    });
                }
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "echo \"Hello, World!\" > output.txt";
        let lexer = TokenLexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Word("echo"));
        assert_eq!(tokens[1].token, Token::String("Hello, World!"));
        assert_eq!(tokens[2].token, Token::RedirectOutput);
        assert_eq!(tokens[3].token, Token::Word("output.txt"));
    }

    #[test]
    fn test_lexer_fail() {
        let input = "";
        let lexer = TokenLexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_lexer_redirect_io() {
        let input = "echo \"Hello, World!\" 2> output.txt";
        let lexer = TokenLexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token, Token::Word("echo"));
        assert_eq!(tokens[1].token, Token::String("Hello, World!"));
        assert_eq!(tokens[2].token, Token::Word("2"));
        assert_eq!(tokens[3].token, Token::RedirectOutput);
        assert_eq!(tokens[4].token, Token::Word("output.txt"));
    }

    #[test]
    fn test_special_string() {
        let input = "echo %{Hello, World!} > output.txt";
        let lexer = TokenLexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Word("echo"));
        assert_eq!(tokens[1].token, Token::String("Hello, World!"));
        assert_eq!(tokens[2].token, Token::RedirectOutput);
        assert_eq!(tokens[3].token, Token::Word("output.txt"));
    }

    #[test]
    fn test_pipe() {
        let input = "echo \"Hello, World!\" | grep World";
        let lexer = TokenLexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token, Token::Word("echo"));
        assert_eq!(tokens[1].token, Token::String("Hello, World!"));
        assert_eq!(tokens[2].token, Token::Pipe);
        assert_eq!(tokens[3].token, Token::Word("grep"));
        assert_eq!(tokens[4].token, Token::Word("World"));
    }

    #[test]
    fn test_and() {
        let input = "echo \"Hello, World!\" && echo \"Hello, World!\"";
        let lexer = TokenLexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token, Token::Word("echo"));
        assert_eq!(tokens[1].token, Token::String("Hello, World!"));
        assert_eq!(tokens[2].token, Token::And);
        assert_eq!(tokens[3].token, Token::Word("echo"));
        assert_eq!(tokens[4].token, Token::String("Hello, World!"));
    }

    #[test]
    fn test_or() {
        let input = "echo \"Hello, World!\" || echo \"Hello, World!\"";
        let lexer = TokenLexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token, Token::Word("echo"));
        assert_eq!(tokens[1].token, Token::String("Hello, World!"));
        assert_eq!(tokens[2].token, Token::Or);
        assert_eq!(tokens[3].token, Token::Word("echo"));
        assert_eq!(tokens[4].token, Token::String("Hello, World!"));
    }

    #[test]
    fn test_semi_colon() {
        let input = "echo \"Hello, World!\"; echo \"Hello, World!\"";
        let lexer = TokenLexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token, Token::Word("echo"));
        assert_eq!(tokens[1].token, Token::String("Hello, World!"));
        assert_eq!(tokens[2].token, Token::SemiColon);
        assert_eq!(tokens[3].token, Token::Word("echo"));
        assert_eq!(tokens[4].token, Token::String("Hello, World!"));
    }

    #[test]
    fn test_redirect_append() {
        let input = "echo \"Hello, World!\" >> output.txt";
        let lexer = TokenLexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Word("echo"));
        assert_eq!(tokens[1].token, Token::String("Hello, World!"));
        assert_eq!(tokens[2].token, Token::RedirectAppend);
        assert_eq!(tokens[3].token, Token::Word("output.txt"));
    }

    #[test]
    fn test_redirect_input() {
        let input = "echo \"Hello, World!\" < input.txt";
        let lexer = TokenLexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Word("echo"));
        assert_eq!(tokens[1].token, Token::String("Hello, World!"));
        assert_eq!(tokens[2].token, Token::RedirectInput);
        assert_eq!(tokens[3].token, Token::Word("input.txt"));
    }

    #[test]
    fn test_redirect_output() {
        let input = "echo \"Hello, World!\" > output.txt";
        let lexer = TokenLexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Word("echo"));
        assert_eq!(tokens[1].token, Token::String("Hello, World!"));
        assert_eq!(tokens[2].token, Token::RedirectOutput);
        assert_eq!(tokens[3].token, Token::Word("output.txt"));
    }
}

