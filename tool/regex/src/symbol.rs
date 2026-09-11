use {
    alloc::{boxed::Box, vec::Vec},
    parser::Parser,
};

#[derive(Parser)]
pub struct Expression(Term, Vec<(VerticalBar, Term)>);

#[derive(Parser)]
pub struct Term(Vec<Power>);

#[derive(Parser)]
pub struct Power(Base, Option<Exponent>);

#[derive(Parser)]
pub enum Base {
    Character(Character),
    Expression(LeftParenthesis, Box<Expression>, RightParenthesis),
    Set(LeftBracket, Option<Circumflex>, Set, RightBracket),
}

#[derive(Parser)]
pub enum Character {
    Circumflex(Circumflex),
    Dollar(Dollar),
    EscapedCharacter(Backslash, EscapedCharacter),
    Period(Period),
    UnescapedCharacter(UnescapedCharacter),
}

#[derive(Parser)]
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

#[derive(Parser)]
pub struct Byte([Hexadecimal; 2]);

#[derive(Parser)]
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

#[derive(Parser)]
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

#[derive(Parser)]
pub struct Set(Vec<Range>);

#[derive(Parser)]
pub struct Range(Element, Option<(Hyphen, Element)>);

#[derive(Parser)]
pub enum Element {
    EscapedElement(Backslash, EscapedElement),
    UnescapedElement(UnescapedElement),
}

#[derive(Parser)]
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

#[derive(Parser)]
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

#[derive(Parser)]
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

#[derive(Parser)]
pub struct Number(Vec<Digit>);

#[derive(Parser)]
pub enum Digit {
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

#[derive(Parser)]
#[terminal = '0']
pub struct Zero;

#[derive(Parser)]
#[terminal = '1']
pub struct One;

#[derive(Parser)]
#[terminal = '2']
pub struct Two;

#[derive(Parser)]
#[terminal = '3']
pub struct Three;

#[derive(Parser)]
#[terminal = '4']
pub struct Four;

#[derive(Parser)]
#[terminal = '5']
pub struct Five;

#[derive(Parser)]
#[terminal = '6']
pub struct Six;

#[derive(Parser)]
#[terminal = '7']
pub struct Seven;

#[derive(Parser)]
#[terminal = '8']
pub struct Eight;

#[derive(Parser)]
#[terminal = '9']
pub struct Nine;

#[derive(Parser)]
#[terminal = 'a']
pub struct LowerA;

#[derive(Parser)]
#[terminal = 'b']
pub struct LowerB;

#[derive(Parser)]
#[terminal = 'c']
pub struct LowerC;

#[derive(Parser)]
#[terminal = 'd']
pub struct LowerD;

#[derive(Parser)]
#[terminal = 'e']
pub struct LowerE;

#[derive(Parser)]
#[terminal = 'f']
pub struct LowerF;

#[derive(Parser)]
#[terminal = 'g']
pub struct LowerG;

#[derive(Parser)]
#[terminal = 'h']
pub struct LowerH;

#[derive(Parser)]
#[terminal = 'i']
pub struct LowerI;

#[derive(Parser)]
#[terminal = 'j']
pub struct LowerJ;

#[derive(Parser)]
#[terminal = 'k']
pub struct LowerK;

#[derive(Parser)]
#[terminal = 'l']
pub struct LowerL;

#[derive(Parser)]
#[terminal = 'm']
pub struct LowerM;

#[derive(Parser)]
#[terminal = 'n']
pub struct LowerN;

#[derive(Parser)]
#[terminal = 'o']
pub struct LowerO;

#[derive(Parser)]
#[terminal = 'p']
pub struct LowerP;

#[derive(Parser)]
#[terminal = 'q']
pub struct LowerQ;

#[derive(Parser)]
#[terminal = 'r']
pub struct LowerR;

#[derive(Parser)]
#[terminal = 's']
pub struct LowerS;

#[derive(Parser)]
#[terminal = 't']
pub struct LowerT;

#[derive(Parser)]
#[terminal = 'u']
pub struct LowerU;

#[derive(Parser)]
#[terminal = 'v']
pub struct LowerV;

#[derive(Parser)]
#[terminal = 'w']
pub struct LowerW;

#[derive(Parser)]
#[terminal = 'x']
pub struct LowerX;

#[derive(Parser)]
#[terminal = 'y']
pub struct LowerY;

#[derive(Parser)]
#[terminal = 'z']
pub struct LowerZ;

#[derive(Parser)]
#[terminal = 'A']
pub struct UpperA;

#[derive(Parser)]
#[terminal = 'B']
pub struct UpperB;

#[derive(Parser)]
#[terminal = 'C']
pub struct UpperC;

#[derive(Parser)]
#[terminal = 'D']
pub struct UpperD;

#[derive(Parser)]
#[terminal = 'E']
pub struct UpperE;

#[derive(Parser)]
#[terminal = 'F']
pub struct UpperF;

#[derive(Parser)]
#[terminal = 'G']
pub struct UpperG;

#[derive(Parser)]
#[terminal = 'H']
pub struct UpperH;

#[derive(Parser)]
#[terminal = 'I']
pub struct UpperI;

#[derive(Parser)]
#[terminal = 'J']
pub struct UpperJ;

#[derive(Parser)]
#[terminal = 'K']
pub struct UpperK;

#[derive(Parser)]
#[terminal = 'L']
pub struct UpperL;

#[derive(Parser)]
#[terminal = 'M']
pub struct UpperM;

#[derive(Parser)]
#[terminal = 'N']
pub struct UpperN;

#[derive(Parser)]
#[terminal = 'O']
pub struct UpperO;

#[derive(Parser)]
#[terminal = 'P']
pub struct UpperP;

#[derive(Parser)]
#[terminal = 'Q']
pub struct UpperQ;

#[derive(Parser)]
#[terminal = 'R']
pub struct UpperR;

#[derive(Parser)]
#[terminal = 'S']
pub struct UpperS;

#[derive(Parser)]
#[terminal = 'T']
pub struct UpperT;

#[derive(Parser)]
#[terminal = 'U']
pub struct UpperU;

#[derive(Parser)]
#[terminal = 'V']
pub struct UpperV;

#[derive(Parser)]
#[terminal = 'W']
pub struct UpperW;

#[derive(Parser)]
#[terminal = 'X']
pub struct UpperX;

#[derive(Parser)]
#[terminal = 'Y']
pub struct UpperY;

#[derive(Parser)]
#[terminal = 'Z']
pub struct UpperZ;

#[derive(Parser)]
#[terminal = '&']
pub struct Ampersand;

#[derive(Parser)]
#[terminal = '\'']
pub struct Apostrophe;

#[derive(Parser)]
#[terminal = '*']
pub struct Asterisk;

#[derive(Parser)]
#[terminal = '@']
pub struct At;

#[derive(Parser)]
#[terminal = '\\']
pub struct Backslash;

#[derive(Parser)]
#[terminal = '^']
pub struct Circumflex;

#[derive(Parser)]
#[terminal = ':']
pub struct Colon;

#[derive(Parser)]
#[terminal = ',']
pub struct Comma;

#[derive(Parser)]
#[terminal = '$']
pub struct Dollar;

#[derive(Parser)]
#[terminal = '=']
pub struct Equal;

#[derive(Parser)]
#[terminal = '!']
pub struct Exclamation;

#[derive(Parser)]
#[terminal = '`']
pub struct Grave;

#[derive(Parser)]
#[terminal = '>']
pub struct GreaterThan;

#[derive(Parser)]
#[terminal = '#']
pub struct Hash;

#[derive(Parser)]
#[terminal = '-']
pub struct Hyphen;

#[derive(Parser)]
#[terminal = '{']
pub struct LeftBrace;

#[derive(Parser)]
#[terminal = '[']
pub struct LeftBracket;

#[derive(Parser)]
#[terminal = '(']
pub struct LeftParenthesis;

#[derive(Parser)]
#[terminal = '<']
pub struct LessThan;

#[derive(Parser)]
#[terminal = '%']
pub struct Percent;

#[derive(Parser)]
#[terminal = '.']
pub struct Period;

#[derive(Parser)]
#[terminal = '+']
pub struct Plus;

#[derive(Parser)]
#[terminal = '?']
pub struct Question;

#[derive(Parser)]
#[terminal = '"']
pub struct Quotation;

#[derive(Parser)]
#[terminal = '}']
pub struct RightBrace;

#[derive(Parser)]
#[terminal = ']']
pub struct RightBracket;

#[derive(Parser)]
#[terminal = ')']
pub struct RightParenthesis;

#[derive(Parser)]
#[terminal = ';']
pub struct Semicolon;

#[derive(Parser)]
#[terminal = '/']
pub struct Slash;

#[derive(Parser)]
#[terminal = '~']
pub struct Tilde;

#[derive(Parser)]
#[terminal = '_']
pub struct Underscore;

#[derive(Parser)]
#[terminal = '|']
pub struct VerticalBar;
