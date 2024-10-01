


mod lexer;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Command<'a> {
    Pipeline(Box<Command<'a>>, Option<(SequenceOp, Box<Command<'a>>)>),
    ComplexCommand {
        command: Box<Command<'a>>,
        redirect_in: Option<&'a str>,
        redirect_out: Option<(&'a str, bool)>,
    },
    SimpleCommand(Vec<&'a str>),
}

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Eq, Ord)]
pub enum SequenceOp {
    And,
    Or,
    Semi,
    Pipe,
}




pub fn parse_interactive<'a>(input: &'a str) -> Command<'a> {
    let lex = lexer::TokenLexer::new(input);
    let tokens = lex.collect::<Vec<_>>();
    let mut parser = Parser::new(tokens);
    parser.parse()
}


pub struct Parser<'a> {
    tokens: Vec<lexer::SpannedToken<'a>>,
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<lexer::SpannedToken<'a>>) -> Self {
        Parser {
            tokens,
            index: 0,
        }
    }

    fn parse(&mut self) -> Command<'a> {
        self.parse_pipeline()
    }

    fn parse_pipeline(&mut self) -> Command<'a> {
        let mut cmd = self.parse_complex_command();
        while self.index < self.tokens.len() {
            match self.tokens[self.index].token {
                lexer::Token::Pipe => {
                    self.index += 1;
                    let next = self.parse_command();
                    cmd = Command::Pipeline(Box::new(cmd), Some((SequenceOp::Pipe, Box::new(next))));
                }
                lexer::Token::SemiColon => {
                    self.index += 1;
                    let next = self.parse_command();
                    cmd = Command::Pipeline(Box::new(cmd), Some((SequenceOp::Semi, Box::new(next))));
                }
                lexer::Token::And => {
                    self.index += 1;
                    let next = self.parse_command();
                    cmd = Command::Pipeline(Box::new(cmd), Some((SequenceOp::And, Box::new(next))));
                }
                lexer::Token::Or => {
                    self.index += 1;
                    let next = self.parse_command();
                    cmd = Command::Pipeline(Box::new(cmd), Some((SequenceOp::Or, Box::new(next))));
                }
                _ => break,
            }
        }
        cmd
    }

    fn parse_complex_command(&mut self) -> Command<'a> {
        let cmd = self.parse_command();
        let mut redirect_in = None;
        let mut redirect_out = None;

        while self.index < self.tokens.len() {
            match self.tokens[self.index].token {
                lexer::Token::RedirectInput => {
                    self.index += 1;
                    if self.index < self.tokens.len() {
                        match self.tokens[self.index].token {
                            lexer::Token::Word(s) => {
                                redirect_in = Some(s);
                                self.index += 1;
                            }
                            _ => break,
                        }
                    }
                }
                lexer::Token::RedirectOutput => {
                    self.index += 1;
                    if self.index < self.tokens.len() {
                        match self.tokens[self.index].token {
                            lexer::Token::Word(s) => {
                                redirect_out = Some((s, false));
                                self.index += 1;
                            }
                            _ => break,
                        }
                    }
                }
                lexer::Token::RedirectAppend => {
                    self.index += 1;
                    if self.index < self.tokens.len() {
                        match self.tokens[self.index].token {
                            lexer::Token::Word(s) => {
                                redirect_out = Some((s, true));
                                self.index += 1;
                            }
                            _ => break,
                        }
                    }
                }
                _ => break,
            }
        }

        Command::ComplexCommand {
            command: Box::new(cmd),
            redirect_in,
            redirect_out,
        }

    }

    fn parse_command(&mut self) -> Command<'a> {
        let mut args = Vec::new();
        while self.index < self.tokens.len() {
            match self.tokens[self.index].token {
                lexer::Token::Pipe | lexer::Token::SemiColon | lexer::Token::And | lexer::Token::Or => break,
                lexer::Token::Word(s) => {
                    args.push(s);
                    self.index += 1;
                }
                _ => break,
            }
        }
        Command::SimpleCommand(args)
    }

}
