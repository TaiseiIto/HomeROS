use {
    crate::Automata,
    alloc::{boxed::Box, vec::Vec},
    core::iter::once,
    parser::Parser,
};

#[derive(Debug, Parser)]
pub struct Expression(Term, Vec<(VerticalBar, Term)>);

impl From<Expression> for Automata {
    fn from(expression: Expression) -> Self {
        let Expression(term, terms) = expression;
        Self::Selection(
            once(term)
                .chain(terms.into_iter().map(|(VerticalBar, term)| term))
                .map(|term| term.into())
                .collect(),
        )
    }
}

#[derive(Debug, Parser)]
pub struct Term(Vec<Power>);

impl From<Term> for Automata {
    fn from(term: Term) -> Self {
        Self::Sequence(term.0.into_iter().map(|power| power.into()).collect())
    }
}

#[derive(Debug, Parser)]
pub struct Power(Base, Option<Exponent>);

impl From<Power> for Automata {
    fn from(power: Power) -> Self {
        let Power(base, exponent) = power;
        if let Some(exponent) = exponent {
            let (min, max): (usize, Option<usize>) = exponent.into();
            Self::Repetition {
                body: Box::new(base.into()),
                min,
                max,
            }
        } else {
            base.into()
        }
    }
}

#[derive(Debug, Parser)]
pub enum Base {
    Character(Character),
    Expression(LeftParenthesis, Box<Expression>, RightParenthesis),
    Set(LeftBracket, Option<Circumflex>, Set, RightBracket),
}

impl From<Base> for Automata {
    fn from(base: Base) -> Self {
        unimplemented!();
    }
}

#[derive(Debug, Parser)]
pub enum Character {
    Circumflex(Circumflex),
    Dollar(Dollar),
    EscapedCharacter(Backslash, EscapedCharacter),
    Period(Period),
    UnescapedCharacter(UnescapedCharacter),
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
    UpperA(UpperA),
    UpperB(UpperB),
    UpperC(UpperC),
    UpperD(UpperD),
    UpperE(UpperE),
    UpperF(UpperF),
}

#[derive(Debug, Parser)]
pub enum UnescapedCharacter {
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
    LowerG(LowerG),
    LowerH(LowerH),
    LowerI(LowerI),
    LowerJ(LowerJ),
    LowerK(LowerK),
    LowerL(LowerL),
    LowerM(LowerM),
    LowerN(LowerN),
    LowerO(LowerO),
    LowerP(LowerP),
    LowerQ(LowerQ),
    LowerR(LowerR),
    LowerS(LowerS),
    LowerT(LowerT),
    LowerU(LowerU),
    LowerV(LowerV),
    LowerW(LowerW),
    LowerX(LowerX),
    LowerY(LowerY),
    LowerZ(LowerZ),
    UpperA(UpperA),
    UpperB(UpperB),
    UpperC(UpperC),
    UpperD(UpperD),
    UpperE(UpperE),
    UpperF(UpperF),
    UpperG(UpperG),
    UpperH(UpperH),
    UpperI(UpperI),
    UpperJ(UpperJ),
    UpperK(UpperK),
    UpperL(UpperL),
    UpperM(UpperM),
    UpperN(UpperN),
    UpperO(UpperO),
    UpperP(UpperP),
    UpperQ(UpperQ),
    UpperR(UpperR),
    UpperS(UpperS),
    UpperT(UpperT),
    UpperU(UpperU),
    UpperV(UpperV),
    UpperW(UpperW),
    UpperX(UpperX),
    UpperY(UpperY),
    UpperZ(UpperZ),
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

#[derive(Debug, Parser)]
pub struct Set(Vec<Range>);

#[derive(Debug, Parser)]
pub struct Range(Element, Option<(Hyphen, Element)>);

#[derive(Debug, Parser)]
pub enum Element {
    EscapedElement(Backslash, EscapedElement),
    UnescapedElement(UnescapedElement),
}

#[derive(Debug, Parser)]
pub enum EscapedElement {
    Asterisk(Asterisk),
    Backslash(Backslash),
    Circumflex(Circumflex),
    Dollar(Dollar),
    Hyphen(Hyphen),
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
pub enum UnescapedElement {
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
    LowerG(LowerG),
    LowerH(LowerH),
    LowerI(LowerI),
    LowerJ(LowerJ),
    LowerK(LowerK),
    LowerL(LowerL),
    LowerM(LowerM),
    LowerN(LowerN),
    LowerO(LowerO),
    LowerP(LowerP),
    LowerQ(LowerQ),
    LowerR(LowerR),
    LowerS(LowerS),
    LowerT(LowerT),
    LowerU(LowerU),
    LowerV(LowerV),
    LowerW(LowerW),
    LowerX(LowerX),
    LowerY(LowerY),
    LowerZ(LowerZ),
    UpperA(UpperA),
    UpperB(UpperB),
    UpperC(UpperC),
    UpperD(UpperD),
    UpperE(UpperE),
    UpperF(UpperF),
    UpperG(UpperG),
    UpperH(UpperH),
    UpperI(UpperI),
    UpperJ(UpperJ),
    UpperK(UpperK),
    UpperL(UpperL),
    UpperM(UpperM),
    UpperN(UpperN),
    UpperO(UpperO),
    UpperP(UpperP),
    UpperQ(UpperQ),
    UpperR(UpperR),
    UpperS(UpperS),
    UpperT(UpperT),
    UpperU(UpperU),
    UpperV(UpperV),
    UpperW(UpperW),
    UpperX(UpperX),
    UpperY(UpperY),
    UpperZ(UpperZ),
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
    LessThan(LessThan),
    Percent(Percent),
    Quotation(Quotation),
    Semicolon(Semicolon),
    Tilde(Tilde),
    Underscore(Underscore),
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

impl From<Exponent> for (usize, Option<usize>) {
    fn from(exponent: Exponent) -> Self {
        match exponent {
            Exponent::Asterisk(_) => (0, None),
            Exponent::Plus(_) => (1, None),
            Exponent::Question(_) => (0, Some(1)),
            Exponent::Range(LeftBrace, min, max, RightBrace) => (
                min.into(),
                max.and_then(|(_, max)| max.map(|max| max.into())),
            ),
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
            .map(|digit| digit.into())
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

impl From<Digit> for usize {
    fn from(digit: Digit) -> usize {
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
