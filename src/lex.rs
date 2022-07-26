use core::iter::Peekable;

pub struct Lexer<I: Iterator>(Peekable<I>);

impl<I: Iterator> Lexer<I> {
    pub fn new(it: I) -> Self {
        Self(it.peekable())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    IntLiteral(u128),
    Sigil(char),
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut c = self.0.next()?;
        while c.is_ascii_whitespace() || c == '/' {
            if c == '/' {
                eprintln!("Entered Comment");
                c = '/';
                if *self.0.peek()? != '/' {
                    break;
                }
                self.0.next();

                while self.0.next()? != '\n' {}
            }
            c = self.0.next()?;
        }

        match c{
            ',' => Some(Token::Sigil(c)),
            c if c.is_alphabetic() || c=='_' => {
                let mut str = String::from(c);
                while let Some(c) = self.0.peek(){
                    if !c.is_ascii_alphanumeric() && *c != '_'{
                        break;
                    }
                    str.push(*c);
                    self.0.next();
                }
                return Some(Token::Ident(str))
            }
            '0' =>{
                match self.0.peek(){
                    Some('x') => {
                        self.0.next();
                        let mut val = 0u128;
                        while let Some(c) = self.0.peek(){
                            if !c.is_ascii_alphanumeric(){
                                break;
                            }else if !c.is_ascii_hexdigit(){
                                panic!("Invalid character {c} in hex literal (note: letters not part of the literal must be separated from it by a whitespace)")
                            }

                            val <<= 4;
                            val |= c.to_digit(16).unwrap() as u128;
                            self.0.next();
                        }
                        Some(Token::IntLiteral(val))
                    }
                    Some('0'..='9') => {
                        let mut val = 0u128;
                        while let Some(c) = self.0.peek(){
                            if !c.is_ascii_alphanumeric(){
                                break;
                            }else if !c.is_ascii_digit(){
                                panic!("Invalid character {c} in decimal literal (note: letters not part of the literal must be separated from it by a whitespace)")
                            }

                            val *= 10;
                            val |= c.to_digit(10).unwrap() as u128;
                            self.0.next();
                        }
                        Some(Token::IntLiteral(val))
                    }
                    Some(x) if x.is_ascii_alphabetic() => panic!("Invalid character {c} in numeric literal (note: letters not part of the literal must be separated from it by a whitespace)"),
                    _ => Some(Token::IntLiteral(0))
                }
            }
            c if c.is_ascii_digit()=> {
                let mut val = c.to_digit(10).unwrap() as u128;
                while let Some(c) = self.0.peek(){
                    if !c.is_ascii_alphanumeric(){
                        break;
                    }else if !c.is_ascii_digit(){
                        panic!("Invalid character {c} in decimal literal (note: letters not part of the literal must be separated from it by a whitespace)")
                    }

                    val *= 10;
                    val |= c.to_digit(10).unwrap() as u128;
                    self.0.next();
                }
                Some(Token::IntLiteral(val))
            }
            c => panic!("Invalid charcter {c}")
        }
    }
}
