#![no_std]

extern crate alloc;

use {alloc::boxed::Box, parser::Parser};

#[derive(Parser)]
struct Expression(Term, Option<(VerticalBar, Box<Expression>)>);

#[derive(Parser)]
struct Term(Power, Option<Box<Term>>);

#[derive(Parser)]
struct Power(Base, Option<Exponent>);

#[derive(Parser)]
enum Base {
    Character(Character),
    Expression(LeftParenthesis, Box<Expression>, RightParenthesis),
    Set(LeftBracket, Option<Circumflex>, Set, RightBracket),
}

#[derive(Parser)]
enum Character {
    Circumflex(Circumflex),
    Dollar(Dollar),
    EscapedCharacter(Backslash, EscapedCharacter),
    Period(Period),
    UnescapedCharacter(UnescapedCharacter),
}

#[derive(Parser)]
enum EscapedCharacter {
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
struct Byte([Hexadecimal; 2]);

#[derive(Parser)]
enum Hexadecimal {
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
enum UnescapedCharacter {
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
struct Set(Range, Option<Box<Set>>);

#[derive(Parser)]
struct Range(Element, Option<(Hyphen, Element)>);

#[derive(Parser)]
enum Element {
    EscapedElement(Backslash, EscapedElement),
    UnescapedElement(UnescapedElement),
}

#[derive(Parser)]
enum EscapedElement {
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
enum UnescapedElement {
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
enum Exponent {
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
struct Number(Digit, Option<Box<Number>>);

#[derive(Parser)]
enum Digit {
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
struct Zero;

#[derive(Parser)]
#[terminal = '1']
struct One;

#[derive(Parser)]
#[terminal = '2']
struct Two;

#[derive(Parser)]
#[terminal = '3']
struct Three;

#[derive(Parser)]
#[terminal = '4']
struct Four;

#[derive(Parser)]
#[terminal = '5']
struct Five;

#[derive(Parser)]
#[terminal = '6']
struct Six;

#[derive(Parser)]
#[terminal = '7']
struct Seven;

#[derive(Parser)]
#[terminal = '8']
struct Eight;

#[derive(Parser)]
#[terminal = '9']
struct Nine;

#[derive(Parser)]
#[terminal = 'a']
struct LowerA;

#[derive(Parser)]
#[terminal = 'b']
struct LowerB;

#[derive(Parser)]
#[terminal = 'c']
struct LowerC;

#[derive(Parser)]
#[terminal = 'd']
struct LowerD;

#[derive(Parser)]
#[terminal = 'e']
struct LowerE;

#[derive(Parser)]
#[terminal = 'f']
struct LowerF;

#[derive(Parser)]
#[terminal = 'g']
struct LowerG;

#[derive(Parser)]
#[terminal = 'h']
struct LowerH;

#[derive(Parser)]
#[terminal = 'i']
struct LowerI;

#[derive(Parser)]
#[terminal = 'j']
struct LowerJ;

#[derive(Parser)]
#[terminal = 'k']
struct LowerK;

#[derive(Parser)]
#[terminal = 'l']
struct LowerL;

#[derive(Parser)]
#[terminal = 'm']
struct LowerM;

#[derive(Parser)]
#[terminal = 'n']
struct LowerN;

#[derive(Parser)]
#[terminal = 'o']
struct LowerO;

#[derive(Parser)]
#[terminal = 'p']
struct LowerP;

#[derive(Parser)]
#[terminal = 'q']
struct LowerQ;

#[derive(Parser)]
#[terminal = 'r']
struct LowerR;

#[derive(Parser)]
#[terminal = 's']
struct LowerS;

#[derive(Parser)]
#[terminal = 't']
struct LowerT;

#[derive(Parser)]
#[terminal = 'u']
struct LowerU;

#[derive(Parser)]
#[terminal = 'v']
struct LowerV;

#[derive(Parser)]
#[terminal = 'w']
struct LowerW;

#[derive(Parser)]
#[terminal = 'x']
struct LowerX;

#[derive(Parser)]
#[terminal = 'y']
struct LowerY;

#[derive(Parser)]
#[terminal = 'z']
struct LowerZ;

#[derive(Parser)]
#[terminal = 'A']
struct UpperA;

#[derive(Parser)]
#[terminal = 'B']
struct UpperB;

#[derive(Parser)]
#[terminal = 'C']
struct UpperC;

#[derive(Parser)]
#[terminal = 'D']
struct UpperD;

#[derive(Parser)]
#[terminal = 'E']
struct UpperE;

#[derive(Parser)]
#[terminal = 'F']
struct UpperF;

#[derive(Parser)]
#[terminal = 'G']
struct UpperG;

#[derive(Parser)]
#[terminal = 'H']
struct UpperH;

#[derive(Parser)]
#[terminal = 'I']
struct UpperI;

#[derive(Parser)]
#[terminal = 'J']
struct UpperJ;

#[derive(Parser)]
#[terminal = 'K']
struct UpperK;

#[derive(Parser)]
#[terminal = 'L']
struct UpperL;

#[derive(Parser)]
#[terminal = 'M']
struct UpperM;

#[derive(Parser)]
#[terminal = 'N']
struct UpperN;

#[derive(Parser)]
#[terminal = 'O']
struct UpperO;

#[derive(Parser)]
#[terminal = 'P']
struct UpperP;

#[derive(Parser)]
#[terminal = 'Q']
struct UpperQ;

#[derive(Parser)]
#[terminal = 'R']
struct UpperR;

#[derive(Parser)]
#[terminal = 'S']
struct UpperS;

#[derive(Parser)]
#[terminal = 'T']
struct UpperT;

#[derive(Parser)]
#[terminal = 'U']
struct UpperU;

#[derive(Parser)]
#[terminal = 'V']
struct UpperV;

#[derive(Parser)]
#[terminal = 'W']
struct UpperW;

#[derive(Parser)]
#[terminal = 'X']
struct UpperX;

#[derive(Parser)]
#[terminal = 'Y']
struct UpperY;

#[derive(Parser)]
#[terminal = 'Z']
struct UpperZ;

#[derive(Parser)]
#[terminal = '&']
struct Ampersand;

#[derive(Parser)]
#[terminal = '\'']
struct Apostrophe;

#[derive(Parser)]
#[terminal = '*']
struct Asterisk;

#[derive(Parser)]
#[terminal = '@']
struct At;

#[derive(Parser)]
#[terminal = '\\']
struct Backslash;

#[derive(Parser)]
#[terminal = '^']
struct Circumflex;

#[derive(Parser)]
#[terminal = ':']
struct Colon;

#[derive(Parser)]
#[terminal = ',']
struct Comma;

#[derive(Parser)]
#[terminal = '$']
struct Dollar;

#[derive(Parser)]
#[terminal = '=']
struct Equal;

#[derive(Parser)]
#[terminal = '!']
struct Exclamation;

#[derive(Parser)]
#[terminal = '`']
struct Grave;

#[derive(Parser)]
#[terminal = '>']
struct GreaterThan;

#[derive(Parser)]
#[terminal = '#']
struct Hash;

#[derive(Parser)]
#[terminal = '-']
struct Hyphen;

#[derive(Parser)]
#[terminal = '{']
struct LeftBrace;

#[derive(Parser)]
#[terminal = '[']
struct LeftBracket;

#[derive(Parser)]
#[terminal = '(']
struct LeftParenthesis;

#[derive(Parser)]
#[terminal = '<']
struct LessThan;

#[derive(Parser)]
#[terminal = '%']
struct Percent;

#[derive(Parser)]
#[terminal = '.']
struct Period;

#[derive(Parser)]
#[terminal = '+']
struct Plus;

#[derive(Parser)]
#[terminal = '?']
struct Question;

#[derive(Parser)]
#[terminal = '"']
struct Quotation;

#[derive(Parser)]
#[terminal = '}']
struct RightBrace;

#[derive(Parser)]
#[terminal = ']']
struct RightBracket;

#[derive(Parser)]
#[terminal = ')']
struct RightParenthesis;

#[derive(Parser)]
#[terminal = ';']
struct Semicolon;

#[derive(Parser)]
#[terminal = '/']
struct Slash;

#[derive(Parser)]
#[terminal = '~']
struct Tilde;

#[derive(Parser)]
#[terminal = '_']
struct Underscore;

#[derive(Parser)]
#[terminal = '|']
struct VerticalBar;
