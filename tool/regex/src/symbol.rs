use {
    crate::{Automaton, automaton, character::Acceptor},
    alloc::{boxed::Box, string::String, vec::Vec},
    core::{iter::once, str::FromStr},
    parser::Parser,
};

#[derive(Debug, Parser)]
pub struct Capturer(Option<CapturerName>, Expression);

impl From<Capturer> for Automaton {
    fn from(capturer: Capturer) -> Self {
        let Capturer(name, expression) = capturer;
        Self::Capturer {
            body: Box::new(expression.into()),
            name: name.map_or(String::default(), |name| name.into()),
        }
    }
}

impl FromStr for Capturer {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::parse(string)
            .and_then(|(capturer, remaining_string)| {
                remaining_string.is_empty().then_some(capturer)
            })
            .ok_or(())
    }
}

#[derive(Debug, Parser)]
pub struct CapturerName(Question, LessThan, Name, GreaterThan);

impl From<CapturerName> for String {
    fn from(capturer_name: CapturerName) -> Self {
        capturer_name.2.into()
    }
}

#[derive(Debug, Parser)]
pub struct Name(Vec<NameCharacter>);

impl From<Name> for String {
    fn from(name: Name) -> Self {
        name.0.into_iter().map(Into::<char>::into).collect()
    }
}

#[derive(Debug, Parser)]
pub enum NameCharacter {
    Digit(Digit),
    Lowercase(Lowercase),
    Uppercase(Uppercase),
    Underscore(Underscore),
}

impl From<NameCharacter> for char {
    fn from(name_character: NameCharacter) -> Self {
        match name_character {
            NameCharacter::Digit(digit) => digit.into(),
            NameCharacter::Lowercase(lowercase) => lowercase.into(),
            NameCharacter::Uppercase(uppercase) => uppercase.into(),
            NameCharacter::Underscore(underscore) => underscore.into(),
        }
    }
}

#[derive(Debug, Parser)]
pub struct Expression(Term, Vec<(VerticalBar, Term)>);

impl From<Expression> for Automaton {
    fn from(expression: Expression) -> Self {
        let mut terms: Vec<Term> = expression.into();
        if terms.len() == 1 {
            terms.pop().unwrap().into()
        } else {
            Self::Selection(terms.into_iter().map(Into::into).collect())
        }
    }
}

impl From<Expression> for Vec<Term> {
    fn from(expression: Expression) -> Self {
        let Expression(term, terms) = expression;
        once(term)
            .chain(terms.into_iter().map(|(_, term)| term))
            .collect()
    }
}

#[derive(Debug, Parser)]
pub struct Term(Vec<Power>);

impl From<Term> for Automaton {
    fn from(term: Term) -> Self {
        let Term(mut powers) = term;
        if powers.len() == 1 {
            powers.pop().unwrap().into()
        } else {
            Self::Sequence(powers.into_iter().map(Into::into).collect())
        }
    }
}

#[derive(Debug, Parser)]
pub struct Power(Base, Option<Exponent>);

impl From<Power> for Automaton {
    fn from(power: Power) -> Self {
        let Power(base, exponent) = power;
        if let Some(exponent) = exponent {
            Self::Repetition {
                body: Box::new(base.into()),
                number: exponent.into(),
            }
        } else {
            base.into()
        }
    }
}

#[derive(Debug, Parser)]
pub enum Base {
    Capturer(LeftParenthesis, Box<Capturer>, RightParenthesis),
    Character(Character),
    Set(LeftBracket, Option<Circumflex>, Set, RightBracket),
}

impl From<Base> for Automaton {
    fn from(base: Base) -> Self {
        match base {
            Base::Capturer(LeftParenthesis, capturer, RightParenthesis) => (*capturer).into(),
            Base::Character(character) => character.into(),
            Base::Set(LeftBracket, circumflex, set, RightBracket) => Self::Character({
                let set: Acceptor = set.into();
                if circumflex.is_some() { -set } else { set }
            }),
        }
    }
}

#[derive(Debug, Parser)]
pub enum Character {
    Circumflex(Circumflex),
    Dollar(Dollar),
    Escaped(Backslash, EscapedCharacter),
    Period(Period),
    Naked(NakedCharacter),
}

impl From<Character> for Automaton {
    fn from(character: Character) -> Self {
        match character {
            Character::Circumflex(Circumflex) => Self::StartOfLine,
            Character::Dollar(Dollar) => Self::EndOfLine,
            Character::Escaped(Backslash, escaped_character) => match escaped_character {
                EscapedCharacter::Asterisk(asterisk) => Into::<char>::into(asterisk).into(),
                EscapedCharacter::Backslash(backslash) => Into::<char>::into(backslash).into(),
                EscapedCharacter::Circumflex(circumflex) => Into::<char>::into(circumflex).into(),
                EscapedCharacter::Dollar(dollar) => Into::<char>::into(dollar).into(),
                EscapedCharacter::LeftBrace(left_brace) => Into::<char>::into(left_brace).into(),
                EscapedCharacter::LeftBracket(left_bracket) => {
                    Into::<char>::into(left_bracket).into()
                }
                EscapedCharacter::LeftParenthesis(left_parenthesis) => {
                    Into::<char>::into(left_parenthesis).into()
                }
                EscapedCharacter::LowerD(LowerD) => {
                    Self::Character(EscapedElement::LowerD(LowerD).into())
                }
                EscapedCharacter::LowerF(LowerF) => Self::Character('\x0C'.into()),
                EscapedCharacter::LowerL(LowerL) => {
                    Self::Character(EscapedElement::LowerL(LowerL).into())
                }
                EscapedCharacter::LowerN(LowerN) => Self::Character('\n'.into()),
                EscapedCharacter::LowerR(LowerR) => Self::Character('\r'.into()),
                EscapedCharacter::LowerS(LowerS) => {
                    Self::Character(EscapedElement::LowerS(LowerS).into())
                }
                EscapedCharacter::LowerT(LowerT) => Self::Character('\t'.into()),
                EscapedCharacter::LowerU(LowerU) => {
                    Self::Character(EscapedElement::LowerU(LowerU).into())
                }
                EscapedCharacter::LowerW(LowerW) => {
                    Self::Character(EscapedElement::LowerW(LowerW).into())
                }
                EscapedCharacter::LowerX(LowerX, byte) => {
                    Self::Character(Into::<char>::into(byte).into())
                }
                EscapedCharacter::Period(period) => Into::<char>::into(period).into(),
                EscapedCharacter::Plus(plus) => Into::<char>::into(plus).into(),
                EscapedCharacter::Question(question) => Into::<char>::into(question).into(),
                EscapedCharacter::RightBrace(right_brace) => Into::<char>::into(right_brace).into(),
                EscapedCharacter::RightBracket(right_bracket) => {
                    Into::<char>::into(right_bracket).into()
                }
                EscapedCharacter::RightParenthesis(right_parenthesis) => {
                    Into::<char>::into(right_parenthesis).into()
                }
                EscapedCharacter::Slash(slash) => Into::<char>::into(slash).into(),
                EscapedCharacter::UpperD(UpperD) => {
                    Self::Character(EscapedElement::UpperD(UpperD).into())
                }
                EscapedCharacter::UpperL(UpperL) => {
                    Self::Character(EscapedElement::UpperL(UpperL).into())
                }
                EscapedCharacter::UpperU(UpperU) => {
                    Self::Character(EscapedElement::UpperU(UpperU).into())
                }
                EscapedCharacter::UpperS(UpperS) => {
                    Self::Character(EscapedElement::UpperS(UpperS).into())
                }
                EscapedCharacter::UpperW(UpperW) => {
                    Self::Character(EscapedElement::UpperW(UpperW).into())
                }
                EscapedCharacter::VerticalBar(vertical_bar) => {
                    Into::<char>::into(vertical_bar).into()
                }
            },
            Character::Period(Period) => Self::Character("".chars().collect()),
            Character::Naked(character) => character.into(),
        }
    }
}

#[derive(Debug, Parser)]
pub enum EscapedCharacter {
    Asterisk(Asterisk),
    Backslash(Backslash),
    Circumflex(Circumflex),
    Dollar(Dollar),
    LeftBrace(LeftBrace),
    LeftBracket(LeftBracket),
    LeftParenthesis(LeftParenthesis),
    LowerD(LowerD),
    LowerF(LowerF),
    LowerL(LowerL),
    LowerN(LowerN),
    LowerR(LowerR),
    LowerS(LowerS),
    LowerT(LowerT),
    LowerU(LowerU),
    LowerW(LowerW),
    LowerX(LowerX, Byte),
    Period(Period),
    Plus(Plus),
    Question(Question),
    RightBrace(RightBrace),
    RightBracket(RightBracket),
    RightParenthesis(RightParenthesis),
    Slash(Slash),
    UpperD(UpperD),
    UpperL(UpperL),
    UpperS(UpperS),
    UpperU(UpperU),
    UpperW(UpperW),
    VerticalBar(VerticalBar),
}

#[derive(Debug, Parser)]
pub struct Byte([Hexadecimal; 2]);

impl From<Byte> for char {
    fn from(byte: Byte) -> Self {
        let byte: u8 = byte.into();
        byte as char
    }
}

impl From<Byte> for u8 {
    fn from(byte: Byte) -> Self {
        let Byte([high_nibble, low_nibble]) = byte;
        let high_nibble: u8 = high_nibble.into();
        let low_nibble: u8 = low_nibble.into();
        0x10 * high_nibble + low_nibble
    }
}

#[derive(Debug, Parser)]
pub enum Hexadecimal {
    Zero(Zero),
    One(One),
    Two(Two),
    Three(Three),
    Four(Four),
    Five(Five),
    Six(Six),
    Seven(Seven),
    Eight(Eight),
    Nine(Nine),
    LowerA(LowerA),
    LowerB(LowerB),
    LowerC(LowerC),
    LowerD(LowerD),
    LowerE(LowerE),
    LowerF(LowerF),
    UpperA(UpperA),
    UpperB(UpperB),
    UpperC(UpperC),
    UpperD(UpperD),
    UpperE(UpperE),
    UpperF(UpperF),
}

impl From<Hexadecimal> for u8 {
    fn from(hexadecimal: Hexadecimal) -> Self {
        match hexadecimal {
            Hexadecimal::Zero(Zero) => 0x00,
            Hexadecimal::One(One) => 0x01,
            Hexadecimal::Two(Two) => 0x02,
            Hexadecimal::Three(Three) => 0x03,
            Hexadecimal::Four(Four) => 0x04,
            Hexadecimal::Five(Five) => 0x05,
            Hexadecimal::Six(Six) => 0x06,
            Hexadecimal::Seven(Seven) => 0x07,
            Hexadecimal::Eight(Eight) => 0x08,
            Hexadecimal::Nine(Nine) => 0x09,
            Hexadecimal::LowerA(LowerA) => 0x0a,
            Hexadecimal::LowerB(LowerB) => 0x0b,
            Hexadecimal::LowerC(LowerC) => 0x0c,
            Hexadecimal::LowerD(LowerD) => 0x0d,
            Hexadecimal::LowerE(LowerE) => 0x0e,
            Hexadecimal::LowerF(LowerF) => 0x0f,
            Hexadecimal::UpperA(UpperA) => 0x0a,
            Hexadecimal::UpperB(UpperB) => 0x0b,
            Hexadecimal::UpperC(UpperC) => 0x0c,
            Hexadecimal::UpperD(UpperD) => 0x0d,
            Hexadecimal::UpperE(UpperE) => 0x0e,
            Hexadecimal::UpperF(UpperF) => 0x0f,
        }
    }
}

#[derive(Debug, Parser)]
pub enum NakedCharacter {
    Zero(Zero),
    One(One),
    Two(Two),
    Three(Three),
    Four(Four),
    Five(Five),
    Six(Six),
    Seven(Seven),
    Eight(Eight),
    Nine(Nine),
    Lowercase(Lowercase),
    Uppercase(Uppercase),
    Ampersand(Ampersand),
    Apostrophe(Apostrophe),
    At(At),
    Colon(Colon),
    Comma(Comma),
    Equal(Equal),
    Exclamation(Exclamation),
    Grave(Grave),
    GreaterThan(GreaterThan),
    Hash(Hash),
    Hyphen(Hyphen),
    LessThan(LessThan),
    Percent(Percent),
    Quotation(Quotation),
    Semicolon(Semicolon),
    Tilde(Tilde),
    Underscore(Underscore),
}

impl From<NakedCharacter> for Automaton {
    fn from(character: NakedCharacter) -> Self {
        let character: char = character.into();
        Automaton::Character(character.into())
    }
}

impl From<NakedCharacter> for char {
    fn from(character: NakedCharacter) -> Self {
        match character {
            NakedCharacter::Zero(zero) => zero.into(),
            NakedCharacter::One(one) => one.into(),
            NakedCharacter::Two(two) => two.into(),
            NakedCharacter::Three(three) => three.into(),
            NakedCharacter::Four(four) => four.into(),
            NakedCharacter::Five(five) => five.into(),
            NakedCharacter::Six(six) => six.into(),
            NakedCharacter::Seven(seven) => seven.into(),
            NakedCharacter::Eight(eight) => eight.into(),
            NakedCharacter::Nine(nine) => nine.into(),
            NakedCharacter::Lowercase(lowercase) => lowercase.into(),
            NakedCharacter::Uppercase(uppercase) => uppercase.into(),
            NakedCharacter::Ampersand(ampersand) => ampersand.into(),
            NakedCharacter::Apostrophe(apostrophe) => apostrophe.into(),
            NakedCharacter::At(at) => at.into(),
            NakedCharacter::Colon(colon) => colon.into(),
            NakedCharacter::Comma(comma) => comma.into(),
            NakedCharacter::Equal(equal) => equal.into(),
            NakedCharacter::Exclamation(exclamation) => exclamation.into(),
            NakedCharacter::Grave(grave) => grave.into(),
            NakedCharacter::GreaterThan(greater_than) => greater_than.into(),
            NakedCharacter::Hash(hash) => hash.into(),
            NakedCharacter::Hyphen(hyphen) => hyphen.into(),
            NakedCharacter::LessThan(less_than) => less_than.into(),
            NakedCharacter::Percent(percent) => percent.into(),
            NakedCharacter::Quotation(quotation) => quotation.into(),
            NakedCharacter::Semicolon(semicolon) => semicolon.into(),
            NakedCharacter::Tilde(tilde) => tilde.into(),
            NakedCharacter::Underscore(underscore) => underscore.into(),
        }
    }
}

#[derive(Debug, Parser)]
pub struct Set(Vec<Range>);

impl From<Set> for Acceptor {
    fn from(set: Set) -> Self {
        set.0.into_iter().map(Into::<Self>::into).sum()
    }
}

#[derive(Debug, Parser)]
pub struct Range(Element, Option<(Hyphen, Element)>);

impl From<Range> for Acceptor {
    fn from(range: Range) -> Self {
        let Range(start, end) = range;
        if let Some((Hyphen, end)) = end {
            let start: u8 = start.into();
            let end: u8 = end.into();
            (start..=end).map(|character| character as char).collect()
        } else {
            start.into()
        }
    }
}

#[derive(Debug, Parser)]
pub enum Element {
    Escaped(Backslash, EscapedElement),
    Naked(NakedElement),
}

impl From<Element> for Acceptor {
    fn from(element: Element) -> Self {
        match element {
            Element::Escaped(Backslash, escaped_element) => escaped_element.into(),
            Element::Naked(naked_element) => naked_element.into(),
        }
    }
}

impl From<Element> for u8 {
    fn from(element: Element) -> Self {
        match element {
            Element::Escaped(Backslash, escaped_element) => escaped_element.into(),
            Element::Naked(naked_element) => naked_element.into(),
        }
    }
}

#[derive(Debug, Parser)]
pub enum EscapedElement {
    Backslash(Backslash),
    Hyphen(Hyphen),
    LowerD(LowerD),
    LowerF(LowerF),
    LowerL(LowerL),
    LowerN(LowerN),
    LowerR(LowerR),
    LowerS(LowerS),
    LowerT(LowerT),
    LowerU(LowerU),
    LowerW(LowerW),
    LowerX(LowerX, Byte),
    RightBracket(RightBracket),
    UpperD(UpperD),
    UpperL(UpperL),
    UpperS(UpperS),
    UpperU(UpperU),
    UpperW(UpperW),
}

impl From<EscapedElement> for Acceptor {
    fn from(escaped_element: EscapedElement) -> Self {
        match escaped_element {
            EscapedElement::Backslash(backslash) => Into::<char>::into(backslash).into(),
            EscapedElement::Hyphen(hyphen) => Into::<char>::into(hyphen).into(),
            EscapedElement::LowerD(LowerD) => "0123456789".chars().collect(),
            EscapedElement::LowerF(LowerF) => '\x0C'.into(),
            EscapedElement::LowerL(LowerL) => "abcdefghijklmnopqrstuvwxyz".chars().collect(),
            EscapedElement::LowerN(LowerN) => '\n'.into(),
            EscapedElement::LowerR(LowerR) => '\r'.into(),
            EscapedElement::LowerS(LowerS) => " \x0C\n\r\t".chars().collect(),
            EscapedElement::LowerT(LowerT) => '\t'.into(),
            EscapedElement::LowerU(LowerU) => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            EscapedElement::LowerW(LowerW) => {
                let lower_d: Self = EscapedElement::LowerD(LowerD).into();
                let lower_l: Self = EscapedElement::LowerL(LowerL).into();
                let lower_u: Self = EscapedElement::LowerU(LowerU).into();
                let underscore: Self = NakedElement::Underscore(Underscore).into();
                lower_d + lower_l + lower_u + underscore
            }
            EscapedElement::LowerX(LowerX, byte) => Into::<char>::into(byte).into(),
            EscapedElement::RightBracket(right_bracket) => Into::<char>::into(right_bracket).into(),
            EscapedElement::UpperD(UpperD) => -Into::<Self>::into(EscapedElement::LowerD(LowerD)),
            EscapedElement::UpperL(UpperL) => -Into::<Self>::into(EscapedElement::LowerL(LowerL)),
            EscapedElement::UpperS(UpperS) => -Into::<Self>::into(EscapedElement::LowerS(LowerS)),
            EscapedElement::UpperU(UpperU) => -Into::<Self>::into(EscapedElement::LowerU(LowerU)),
            EscapedElement::UpperW(UpperW) => -Into::<Self>::into(EscapedElement::LowerW(LowerW)),
        }
    }
}

impl From<EscapedElement> for char {
    fn from(escaped_element: EscapedElement) -> Self {
        match escaped_element {
            EscapedElement::Backslash(backslash) => backslash.into(),
            EscapedElement::Hyphen(hyphen) => hyphen.into(),
            EscapedElement::LowerX(LowerX, byte) => byte.into(),
            EscapedElement::RightBracket(right_bracket) => right_bracket.into(),
            _ => panic!(),
        }
    }
}

impl From<EscapedElement> for u8 {
    fn from(escaped_element: EscapedElement) -> Self {
        match escaped_element {
            EscapedElement::LowerX(LowerX, byte) => byte.into(),
            escaped_element => {
                let escaped_element: char = escaped_element.into();
                escaped_element as Self
            }
        }
    }
}

#[derive(Debug, Parser)]
pub enum NakedElement {
    Zero(Zero),
    One(One),
    Two(Two),
    Three(Three),
    Four(Four),
    Five(Five),
    Six(Six),
    Seven(Seven),
    Eight(Eight),
    Nine(Nine),
    Lowercase(Lowercase),
    Uppercase(Uppercase),
    Ampersand(Ampersand),
    Apostrophe(Apostrophe),
    Asterisk(Asterisk),
    At(At),
    Circumflex(Circumflex),
    Colon(Colon),
    Comma(Comma),
    Dollar(Dollar),
    Equal(Equal),
    Exclamation(Exclamation),
    Grave(Grave),
    GreaterThan(GreaterThan),
    Hash(Hash),
    LeftBrace(LeftBrace),
    LeftBracket(LeftBracket),
    LeftParenthesis(LeftParenthesis),
    LessThan(LessThan),
    Percent(Percent),
    Period(Period),
    Plus(Plus),
    Question(Question),
    Quotation(Quotation),
    RightBrace(RightBrace),
    RightParenthesis(RightParenthesis),
    Semicolon(Semicolon),
    Slash(Slash),
    Tilde(Tilde),
    Underscore(Underscore),
    VerticalBar(VerticalBar),
}

impl From<NakedElement> for Acceptor {
    fn from(naked_element: NakedElement) -> Self {
        let naked_element: char = naked_element.into();
        naked_element.into()
    }
}

impl From<NakedElement> for char {
    fn from(naked_element: NakedElement) -> Self {
        match naked_element {
            NakedElement::Zero(zero) => zero.into(),
            NakedElement::One(one) => one.into(),
            NakedElement::Two(two) => two.into(),
            NakedElement::Three(three) => three.into(),
            NakedElement::Four(four) => four.into(),
            NakedElement::Five(five) => five.into(),
            NakedElement::Six(six) => six.into(),
            NakedElement::Seven(seven) => seven.into(),
            NakedElement::Eight(eight) => eight.into(),
            NakedElement::Nine(nine) => nine.into(),
            NakedElement::Lowercase(lowercase) => lowercase.into(),
            NakedElement::Uppercase(uppercase) => uppercase.into(),
            NakedElement::Ampersand(ampersand) => ampersand.into(),
            NakedElement::Apostrophe(apostrophe) => apostrophe.into(),
            NakedElement::Asterisk(asterisk) => asterisk.into(),
            NakedElement::At(at) => at.into(),
            NakedElement::Circumflex(circumflex) => circumflex.into(),
            NakedElement::Colon(colon) => colon.into(),
            NakedElement::Comma(comma) => comma.into(),
            NakedElement::Dollar(dollar) => dollar.into(),
            NakedElement::Equal(equal) => equal.into(),
            NakedElement::Exclamation(exclamation) => exclamation.into(),
            NakedElement::Grave(grave) => grave.into(),
            NakedElement::GreaterThan(greater_than) => greater_than.into(),
            NakedElement::Hash(hash) => hash.into(),
            NakedElement::LeftBrace(left_brace) => left_brace.into(),
            NakedElement::LeftBracket(left_bracket) => left_bracket.into(),
            NakedElement::LeftParenthesis(left_parenthesis) => left_parenthesis.into(),
            NakedElement::LessThan(less_than) => less_than.into(),
            NakedElement::Percent(percent) => percent.into(),
            NakedElement::Period(period) => period.into(),
            NakedElement::Plus(plus) => plus.into(),
            NakedElement::Question(question) => question.into(),
            NakedElement::Quotation(quotation) => quotation.into(),
            NakedElement::RightBrace(right_brace) => right_brace.into(),
            NakedElement::RightParenthesis(right_parenthesis) => right_parenthesis.into(),
            NakedElement::Semicolon(semicolon) => semicolon.into(),
            NakedElement::Slash(slash) => slash.into(),
            NakedElement::Tilde(tilde) => tilde.into(),
            NakedElement::Underscore(underscore) => underscore.into(),
            NakedElement::VerticalBar(vertical_bar) => vertical_bar.into(),
        }
    }
}

impl From<NakedElement> for u8 {
    fn from(naked_element: NakedElement) -> Self {
        let naked_element: char = naked_element.into();
        naked_element as Self
    }
}

#[derive(Debug, Parser)]
pub enum Exponent {
    Asterisk(Asterisk),
    Plus(Plus),
    Question(Question),
    Range(
        LeftBrace,
        Number,
        Option<(Comma, Option<Number>)>,
        RightBrace,
    ),
}

impl From<Exponent> for automaton::RepetitionNumber {
    fn from(exponent: Exponent) -> Self {
        match exponent {
            Exponent::Asterisk(_) => Self::From(0),
            Exponent::Plus(_) => Self::From(1),
            Exponent::Question(_) => Self::FromTo(0, 1),
            Exponent::Range(LeftBrace, constant, None, RightBrace) => {
                Self::Constant(constant.into())
            }
            Exponent::Range(LeftBrace, min, Some((Comma, None)), RightBrace) => {
                Self::From(min.into())
            }
            Exponent::Range(LeftBrace, min, Some((Comma, Some(max))), RightBrace) => {
                Self::FromTo(min.into(), max.into())
            }
        }
    }
}

#[derive(Debug, Parser)]
pub struct Number(Vec<Digit>);

impl From<Number> for usize {
    fn from(number: Number) -> Self {
        number
            .0
            .into_iter()
            .map(Into::into)
            .fold(0, |number, digit: usize| 10 * number + digit)
    }
}

#[derive(Debug, Parser)]
pub enum Digit {
    Zero(Zero),
    One(One),
    Two(Two),
    Three(Three),
    Four(Four),
    Five(Five),
    Six(Six),
    Seven(Seven),
    Eight(Eight),
    Nine(Nine),
}

impl From<Digit> for char {
    fn from(digit: Digit) -> Self {
        match digit {
            Digit::Zero(zero) => zero.into(),
            Digit::One(one) => one.into(),
            Digit::Two(two) => two.into(),
            Digit::Three(three) => three.into(),
            Digit::Four(four) => four.into(),
            Digit::Five(five) => five.into(),
            Digit::Six(six) => six.into(),
            Digit::Seven(seven) => seven.into(),
            Digit::Eight(eight) => eight.into(),
            Digit::Nine(nine) => nine.into(),
        }
    }
}

impl From<Digit> for usize {
    fn from(digit: Digit) -> Self {
        match digit {
            Digit::Zero(_) => 0,
            Digit::One(_) => 1,
            Digit::Two(_) => 2,
            Digit::Three(_) => 3,
            Digit::Four(_) => 4,
            Digit::Five(_) => 5,
            Digit::Six(_) => 6,
            Digit::Seven(_) => 7,
            Digit::Eight(_) => 8,
            Digit::Nine(_) => 9,
        }
    }
}

#[derive(Debug, Parser)]
pub enum Lowercase {
    A(LowerA),
    B(LowerB),
    C(LowerC),
    D(LowerD),
    E(LowerE),
    F(LowerF),
    G(LowerG),
    H(LowerH),
    I(LowerI),
    J(LowerJ),
    K(LowerK),
    L(LowerL),
    M(LowerM),
    N(LowerN),
    O(LowerO),
    P(LowerP),
    Q(LowerQ),
    R(LowerR),
    S(LowerS),
    T(LowerT),
    U(LowerU),
    V(LowerV),
    W(LowerW),
    X(LowerX),
    Y(LowerY),
    Z(LowerZ),
}

impl From<Lowercase> for char {
    fn from(lowercase: Lowercase) -> Self {
        match lowercase {
            Lowercase::A(a) => a.into(),
            Lowercase::B(b) => b.into(),
            Lowercase::C(c) => c.into(),
            Lowercase::D(d) => d.into(),
            Lowercase::E(e) => e.into(),
            Lowercase::F(f) => f.into(),
            Lowercase::G(g) => g.into(),
            Lowercase::H(h) => h.into(),
            Lowercase::I(i) => i.into(),
            Lowercase::J(j) => j.into(),
            Lowercase::K(k) => k.into(),
            Lowercase::L(l) => l.into(),
            Lowercase::M(m) => m.into(),
            Lowercase::N(n) => n.into(),
            Lowercase::O(o) => o.into(),
            Lowercase::P(p) => p.into(),
            Lowercase::Q(q) => q.into(),
            Lowercase::R(r) => r.into(),
            Lowercase::S(s) => s.into(),
            Lowercase::T(t) => t.into(),
            Lowercase::U(u) => u.into(),
            Lowercase::V(v) => v.into(),
            Lowercase::W(w) => w.into(),
            Lowercase::X(x) => x.into(),
            Lowercase::Y(y) => y.into(),
            Lowercase::Z(z) => z.into(),
        }
    }
}

#[derive(Debug, Parser)]
pub enum Uppercase {
    A(UpperA),
    B(UpperB),
    C(UpperC),
    D(UpperD),
    E(UpperE),
    F(UpperF),
    G(UpperG),
    H(UpperH),
    I(UpperI),
    J(UpperJ),
    K(UpperK),
    L(UpperL),
    M(UpperM),
    N(UpperN),
    O(UpperO),
    P(UpperP),
    Q(UpperQ),
    R(UpperR),
    S(UpperS),
    T(UpperT),
    U(UpperU),
    V(UpperV),
    W(UpperW),
    X(UpperX),
    Y(UpperY),
    Z(UpperZ),
}

impl From<Uppercase> for char {
    fn from(uppercase: Uppercase) -> Self {
        match uppercase {
            Uppercase::A(a) => a.into(),
            Uppercase::B(b) => b.into(),
            Uppercase::C(c) => c.into(),
            Uppercase::D(d) => d.into(),
            Uppercase::E(e) => e.into(),
            Uppercase::F(f) => f.into(),
            Uppercase::G(g) => g.into(),
            Uppercase::H(h) => h.into(),
            Uppercase::I(i) => i.into(),
            Uppercase::J(j) => j.into(),
            Uppercase::K(k) => k.into(),
            Uppercase::L(l) => l.into(),
            Uppercase::M(m) => m.into(),
            Uppercase::N(n) => n.into(),
            Uppercase::O(o) => o.into(),
            Uppercase::P(p) => p.into(),
            Uppercase::Q(q) => q.into(),
            Uppercase::R(r) => r.into(),
            Uppercase::S(s) => s.into(),
            Uppercase::T(t) => t.into(),
            Uppercase::U(u) => u.into(),
            Uppercase::V(v) => v.into(),
            Uppercase::W(w) => w.into(),
            Uppercase::X(x) => x.into(),
            Uppercase::Y(y) => y.into(),
            Uppercase::Z(z) => z.into(),
        }
    }
}

#[derive(Debug, Parser)]
#[terminal = '0']
pub struct Zero;

#[derive(Debug, Parser)]
#[terminal = '1']
pub struct One;

#[derive(Debug, Parser)]
#[terminal = '2']
pub struct Two;

#[derive(Debug, Parser)]
#[terminal = '3']
pub struct Three;

#[derive(Debug, Parser)]
#[terminal = '4']
pub struct Four;

#[derive(Debug, Parser)]
#[terminal = '5']
pub struct Five;

#[derive(Debug, Parser)]
#[terminal = '6']
pub struct Six;

#[derive(Debug, Parser)]
#[terminal = '7']
pub struct Seven;

#[derive(Debug, Parser)]
#[terminal = '8']
pub struct Eight;

#[derive(Debug, Parser)]
#[terminal = '9']
pub struct Nine;

#[derive(Debug, Parser)]
#[terminal = 'a']
pub struct LowerA;

#[derive(Debug, Parser)]
#[terminal = 'b']
pub struct LowerB;

#[derive(Debug, Parser)]
#[terminal = 'c']
pub struct LowerC;

#[derive(Debug, Parser)]
#[terminal = 'd']
pub struct LowerD;

#[derive(Debug, Parser)]
#[terminal = 'e']
pub struct LowerE;

#[derive(Debug, Parser)]
#[terminal = 'f']
pub struct LowerF;

#[derive(Debug, Parser)]
#[terminal = 'g']
pub struct LowerG;

#[derive(Debug, Parser)]
#[terminal = 'h']
pub struct LowerH;

#[derive(Debug, Parser)]
#[terminal = 'i']
pub struct LowerI;

#[derive(Debug, Parser)]
#[terminal = 'j']
pub struct LowerJ;

#[derive(Debug, Parser)]
#[terminal = 'k']
pub struct LowerK;

#[derive(Debug, Parser)]
#[terminal = 'l']
pub struct LowerL;

#[derive(Debug, Parser)]
#[terminal = 'm']
pub struct LowerM;

#[derive(Debug, Parser)]
#[terminal = 'n']
pub struct LowerN;

#[derive(Debug, Parser)]
#[terminal = 'o']
pub struct LowerO;

#[derive(Debug, Parser)]
#[terminal = 'p']
pub struct LowerP;

#[derive(Debug, Parser)]
#[terminal = 'q']
pub struct LowerQ;

#[derive(Debug, Parser)]
#[terminal = 'r']
pub struct LowerR;

#[derive(Debug, Parser)]
#[terminal = 's']
pub struct LowerS;

#[derive(Debug, Parser)]
#[terminal = 't']
pub struct LowerT;

#[derive(Debug, Parser)]
#[terminal = 'u']
pub struct LowerU;

#[derive(Debug, Parser)]
#[terminal = 'v']
pub struct LowerV;

#[derive(Debug, Parser)]
#[terminal = 'w']
pub struct LowerW;

#[derive(Debug, Parser)]
#[terminal = 'x']
pub struct LowerX;

#[derive(Debug, Parser)]
#[terminal = 'y']
pub struct LowerY;

#[derive(Debug, Parser)]
#[terminal = 'z']
pub struct LowerZ;

#[derive(Debug, Parser)]
#[terminal = 'A']
pub struct UpperA;

#[derive(Debug, Parser)]
#[terminal = 'B']
pub struct UpperB;

#[derive(Debug, Parser)]
#[terminal = 'C']
pub struct UpperC;

#[derive(Debug, Parser)]
#[terminal = 'D']
pub struct UpperD;

#[derive(Debug, Parser)]
#[terminal = 'E']
pub struct UpperE;

#[derive(Debug, Parser)]
#[terminal = 'F']
pub struct UpperF;

#[derive(Debug, Parser)]
#[terminal = 'G']
pub struct UpperG;

#[derive(Debug, Parser)]
#[terminal = 'H']
pub struct UpperH;

#[derive(Debug, Parser)]
#[terminal = 'I']
pub struct UpperI;

#[derive(Debug, Parser)]
#[terminal = 'J']
pub struct UpperJ;

#[derive(Debug, Parser)]
#[terminal = 'K']
pub struct UpperK;

#[derive(Debug, Parser)]
#[terminal = 'L']
pub struct UpperL;

#[derive(Debug, Parser)]
#[terminal = 'M']
pub struct UpperM;

#[derive(Debug, Parser)]
#[terminal = 'N']
pub struct UpperN;

#[derive(Debug, Parser)]
#[terminal = 'O']
pub struct UpperO;

#[derive(Debug, Parser)]
#[terminal = 'P']
pub struct UpperP;

#[derive(Debug, Parser)]
#[terminal = 'Q']
pub struct UpperQ;

#[derive(Debug, Parser)]
#[terminal = 'R']
pub struct UpperR;

#[derive(Debug, Parser)]
#[terminal = 'S']
pub struct UpperS;

#[derive(Debug, Parser)]
#[terminal = 'T']
pub struct UpperT;

#[derive(Debug, Parser)]
#[terminal = 'U']
pub struct UpperU;

#[derive(Debug, Parser)]
#[terminal = 'V']
pub struct UpperV;

#[derive(Debug, Parser)]
#[terminal = 'W']
pub struct UpperW;

#[derive(Debug, Parser)]
#[terminal = 'X']
pub struct UpperX;

#[derive(Debug, Parser)]
#[terminal = 'Y']
pub struct UpperY;

#[derive(Debug, Parser)]
#[terminal = 'Z']
pub struct UpperZ;

#[derive(Debug, Parser)]
#[terminal = '&']
pub struct Ampersand;

#[derive(Debug, Parser)]
#[terminal = '\'']
pub struct Apostrophe;

#[derive(Debug, Parser)]
#[terminal = '*']
pub struct Asterisk;

#[derive(Debug, Parser)]
#[terminal = '@']
pub struct At;

#[derive(Debug, Parser)]
#[terminal = '\\']
pub struct Backslash;

#[derive(Debug, Parser)]
#[terminal = '^']
pub struct Circumflex;

#[derive(Debug, Parser)]
#[terminal = ':']
pub struct Colon;

#[derive(Debug, Parser)]
#[terminal = ',']
pub struct Comma;

#[derive(Debug, Parser)]
#[terminal = '$']
pub struct Dollar;

#[derive(Debug, Parser)]
#[terminal = '=']
pub struct Equal;

#[derive(Debug, Parser)]
#[terminal = '!']
pub struct Exclamation;

#[derive(Debug, Parser)]
#[terminal = '`']
pub struct Grave;

#[derive(Debug, Parser)]
#[terminal = '>']
pub struct GreaterThan;

#[derive(Debug, Parser)]
#[terminal = '#']
pub struct Hash;

#[derive(Debug, Parser)]
#[terminal = '-']
pub struct Hyphen;

#[derive(Debug, Parser)]
#[terminal = '{']
pub struct LeftBrace;

#[derive(Debug, Parser)]
#[terminal = '[']
pub struct LeftBracket;

#[derive(Debug, Parser)]
#[terminal = '(']
pub struct LeftParenthesis;

#[derive(Debug, Parser)]
#[terminal = '<']
pub struct LessThan;

#[derive(Debug, Parser)]
#[terminal = '%']
pub struct Percent;

#[derive(Debug, Parser)]
#[terminal = '.']
pub struct Period;

#[derive(Debug, Parser)]
#[terminal = '+']
pub struct Plus;

#[derive(Debug, Parser)]
#[terminal = '?']
pub struct Question;

#[derive(Debug, Parser)]
#[terminal = '"']
pub struct Quotation;

#[derive(Debug, Parser)]
#[terminal = '}']
pub struct RightBrace;

#[derive(Debug, Parser)]
#[terminal = ']']
pub struct RightBracket;

#[derive(Debug, Parser)]
#[terminal = ')']
pub struct RightParenthesis;

#[derive(Debug, Parser)]
#[terminal = ';']
pub struct Semicolon;

#[derive(Debug, Parser)]
#[terminal = '/']
pub struct Slash;

#[derive(Debug, Parser)]
#[terminal = '~']
pub struct Tilde;

#[derive(Debug, Parser)]
#[terminal = '_']
pub struct Underscore;

#[derive(Debug, Parser)]
#[terminal = '|']
pub struct VerticalBar;
