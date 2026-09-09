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
struct One;

#[derive(Parser)]
struct Two;

#[derive(Parser)]
struct Three;

#[derive(Parser)]
struct Four;

#[derive(Parser)]
struct Five;

#[derive(Parser)]
struct Six;

#[derive(Parser)]
struct Seven;

#[derive(Parser)]
struct Eight;

#[derive(Parser)]
struct Nine;

#[derive(Parser)]
struct LowerA;

#[derive(Parser)]
struct LowerB;

#[derive(Parser)]
struct LowerC;

#[derive(Parser)]
struct LowerD;

#[derive(Parser)]
struct LowerE;

#[derive(Parser)]
struct LowerF;

#[derive(Parser)]
struct LowerG;

#[derive(Parser)]
struct LowerH;

#[derive(Parser)]
struct LowerI;

#[derive(Parser)]
struct LowerJ;

#[derive(Parser)]
struct LowerK;

#[derive(Parser)]
struct LowerL;

#[derive(Parser)]
struct LowerM;

#[derive(Parser)]
struct LowerN;

#[derive(Parser)]
struct LowerO;

#[derive(Parser)]
struct LowerP;

#[derive(Parser)]
struct LowerQ;

#[derive(Parser)]
struct LowerR;

#[derive(Parser)]
struct LowerS;

#[derive(Parser)]
struct LowerT;

#[derive(Parser)]
struct LowerU;

#[derive(Parser)]
struct LowerV;

#[derive(Parser)]
struct LowerW;

#[derive(Parser)]
struct LowerX;

#[derive(Parser)]
struct LowerY;

#[derive(Parser)]
struct LowerZ;

#[derive(Parser)]
struct UpperA;

#[derive(Parser)]
struct UpperB;

#[derive(Parser)]
struct UpperC;

#[derive(Parser)]
struct UpperD;

#[derive(Parser)]
struct UpperE;

#[derive(Parser)]
struct UpperF;

#[derive(Parser)]
struct UpperG;

#[derive(Parser)]
struct UpperH;

#[derive(Parser)]
struct UpperI;

#[derive(Parser)]
struct UpperJ;

#[derive(Parser)]
struct UpperK;

#[derive(Parser)]
struct UpperL;

#[derive(Parser)]
struct UpperM;

#[derive(Parser)]
struct UpperN;

#[derive(Parser)]
struct UpperO;

#[derive(Parser)]
struct UpperP;

#[derive(Parser)]
struct UpperQ;

#[derive(Parser)]
struct UpperR;

#[derive(Parser)]
struct UpperS;

#[derive(Parser)]
struct UpperT;

#[derive(Parser)]
struct UpperU;

#[derive(Parser)]
struct UpperV;

#[derive(Parser)]
struct UpperW;

#[derive(Parser)]
struct UpperX;

#[derive(Parser)]
struct UpperY;

#[derive(Parser)]
struct UpperZ;

#[derive(Parser)]
struct Ampersand;

#[derive(Parser)]
struct Apostrophe;

#[derive(Parser)]
struct Asterisk;

#[derive(Parser)]
struct At;

#[derive(Parser)]
struct Backslash;

#[derive(Parser)]
struct Circumflex;

#[derive(Parser)]
struct Colon;

#[derive(Parser)]
struct Comma;

#[derive(Parser)]
struct Dollar;

#[derive(Parser)]
struct Equal;

#[derive(Parser)]
struct Exclamation;

#[derive(Parser)]
struct Grave;

#[derive(Parser)]
struct GreaterThan;

#[derive(Parser)]
struct Hash;

#[derive(Parser)]
struct Hyphen;

#[derive(Parser)]
struct LeftBrace;

#[derive(Parser)]
struct LeftBracket;

#[derive(Parser)]
struct LeftParenthesis;

#[derive(Parser)]
struct LessThan;

#[derive(Parser)]
struct Percent;

#[derive(Parser)]
struct Period;

#[derive(Parser)]
struct Plus;

#[derive(Parser)]
struct Question;

#[derive(Parser)]
struct Quotation;

#[derive(Parser)]
struct RightBrace;

#[derive(Parser)]
struct RightBracket;

#[derive(Parser)]
struct RightParenthesis;

#[derive(Parser)]
struct Semicolon;

#[derive(Parser)]
struct Slash;

#[derive(Parser)]
struct Tilde;

#[derive(Parser)]
struct Underscore;

#[derive(Parser)]
struct VerticalBar;
