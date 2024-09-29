use std::str::CharIndices;

pub struct SpannedToken<'a> {
    pub token: Token<'a>,
    pub start: usize,
    pub end: usize,
}

pub enum Token<'a> {
    Word(&'a str),
    Flag(&'a str),
    String(&'a str),
    Pipe,
    RedirectOutput,
    RedirectAppend,
    RedirectInput,
    RedirectIO(usize, usize),
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
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    while let Some(&(i, c)) = self.input.peek() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                end = i;
                                self.input.next();
                            }
                            _ => break,
                        }
                    }

                    return Some(SpannedToken {
                        token: Token::Word(&self.string[start..=end]),
                        start,
                        end,
                    });
                }
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
                    let Some(&(i, c)) = self.input.peek() else {
                        return None;
                    };
                    let end = match c {
                        '(' => ')',
                        '{' => '}',
                        '[' => ']',
                        c => c,
                    };
                    self.input.next();

                    while let Some(&(i, c)) = self.input.peek() {
                        match c {
                            c if c == end => {
                                self.input.next();
                                return Some(SpannedToken {
                                    token: Token::String(&self.string[start..=i]),
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
                '-' => {
                    while let Some(&(i, c)) = self.input.peek() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => {
                                end = i;
                                self.input.next();
                            }
                            _ => break,
                        }
                    }

                    return Some(SpannedToken {
                        token: Token::Flag(&self.string[start..=end]),
                        start,
                        end,
                    });
                }
                '|' => {
                    if let Some(&(i, c)) = self.input.peek() {
                        if c == '|' {
                            self.input.next();
                            return Some(SpannedToken {
                                token: Token::Or,
                                start,
                                end: i,
                            });
                        }
                    } else {
                        return Some(SpannedToken {
                            token: Token::Pipe,
                            start,
                            end,
                        });
                    }
                }
                '>' => {
                    if let Some(&(i, c)) = self.input.peek() {
                        if c == '>' {
                            self.input.next();
                            return Some(SpannedToken {
                                token: Token::RedirectAppend,
                                start,
                                end: i,
                            });
                        }
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
                _ => {}
            }
        }
        None
    }
}
