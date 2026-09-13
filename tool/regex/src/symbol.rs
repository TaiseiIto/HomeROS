use {
    crate::{Automata, character},
    alloc::{boxed::Box, collections::btree_set::BTreeSet, vec::Vec},
    core::{iter::once, str::FromStr},
    parser::Parser,
};

#[derive(Debug, Parser)]
pub struct Expression(Term, Vec<(VerticalBar, Term)>);

impl FromStr for Expression {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::parse(string)
            .and_then(|(expression, remaining_string)| {
                remaining_string.is_empty().then_some(expression)
            })
            .ok_or(())
    }
}

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
        match base {
            Base::Character(character) => character.into(),
            Base::Expression(LeftParenthesis, expression, RightParenthesis) => (*expression).into(),
            Base::Set(LeftBracket, circumflex, set, RightBracket) => Self::Character({
                let set: character::Set = set.into();
                if circumflex.is_some() { -set } else { set }
            }),
        }
    }
}

#[derive(Debug, Parser)]
pub enum Character {
    Circumflex(Circumflex),
    Dollar(Dollar),
    EscapedCharacter(Backslash, EscapedCharacter),
    Period(Period),
    NakedCharacter(NakedCharacter),
}

impl From<Character> for Automata {
    fn from(character: Character) -> Self {
        match character {
            Character::Circumflex(Circumflex) => Self::StartOfLine,
            Character::Dollar(Dollar) => Self::EndOfLine,
            Character::EscapedCharacter(Backslack, escaped_character) => match escaped_character {
                EscapedCharacter::Asterisk(asterisk) => asterisk.into(),
                EscapedCharacter::Backslash(backslash) => backslash.into(),
                EscapedCharacter::Circumflex(circumflex) => circumflex.into(),
                EscapedCharacter::Dollar(dollar) => dollar.into(),
                EscapedCharacter::LeftBrace(left_brace) => left_brace.into(),
                EscapedCharacter::LeftBracket(left_bracket) => left_bracket.into(),
                EscapedCharacter::LeftParenthesis(left_parenthesis) => left_parenthesis.into(),
                EscapedCharacter::LowerD(LowerD) => r"[\d]".parse().unwrap(),
                EscapedCharacter::LowerF(LowerF) => Self::Character('\x0C'.into()),
                EscapedCharacter::LowerL(LowerL) => r"[\l]".parse().unwrap(),
                EscapedCharacter::LowerN(LowerN) => Self::Character('\n'.into()),
                EscapedCharacter::LowerR(LowerR) => Self::Character('\r'.into()),
                EscapedCharacter::LowerS(LowerS) => r"[\s]".parse().unwrap(),
                EscapedCharacter::LowerT(LowerT) => Self::Character('\t'.into()),
                EscapedCharacter::LowerU(LowerU) => r"[\u]".parse().unwrap(),
                EscapedCharacter::LowerW(LowerW) => r"[\w]".parse().unwrap(),
                EscapedCharacter::LowerX(LowerX, byte) => Self::Character(byte.into()),
                EscapedCharacter::Period(period) => period.into(),
                EscapedCharacter::Plus(plus) => plus.into(),
                EscapedCharacter::Question(question) => question.into(),
                EscapedCharacter::RightBrace(right_brace) => right_brace.into(),
                EscapedCharacter::RightBracket(right_bracket) => right_bracket.into(),
                EscapedCharacter::RightParenthesis(right_parenthesis) => right_parenthesis.into(),
                EscapedCharacter::Slash(slash) => slash.into(),
                EscapedCharacter::UpperD(UpperD) => r"[\D]".parse().unwrap(),
                EscapedCharacter::UpperL(UpperL) => r"[\L]".parse().unwrap(),
                EscapedCharacter::UpperU(UpperU) => r"[\U]".parse().unwrap(),
                EscapedCharacter::UpperS(UpperS) => r"[\S]".parse().unwrap(),
                EscapedCharacter::UpperW(UpperW) => r"[\W]".parse().unwrap(),
                EscapedCharacter::VerticalBar(vertical_bar) => vertical_bar.into(),
            },
            Character::Period(Period) => r"[^]".parse().unwrap(),
            Character::NakedCharacter(character) => character.into(),
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

impl From<NakedCharacter> for Automata {
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
            NakedCharacter::LowerA(lower_a) => lower_a.into(),
            NakedCharacter::LowerB(lower_b) => lower_b.into(),
            NakedCharacter::LowerC(lower_c) => lower_c.into(),
            NakedCharacter::LowerD(lower_d) => lower_d.into(),
            NakedCharacter::LowerE(lower_e) => lower_e.into(),
            NakedCharacter::LowerF(lower_f) => lower_f.into(),
            NakedCharacter::LowerG(lower_g) => lower_g.into(),
            NakedCharacter::LowerH(lower_h) => lower_h.into(),
            NakedCharacter::LowerI(lower_i) => lower_i.into(),
            NakedCharacter::LowerJ(lower_j) => lower_j.into(),
            NakedCharacter::LowerK(lower_k) => lower_k.into(),
            NakedCharacter::LowerL(lower_l) => lower_l.into(),
            NakedCharacter::LowerM(lower_m) => lower_m.into(),
            NakedCharacter::LowerN(lower_n) => lower_n.into(),
            NakedCharacter::LowerO(lower_o) => lower_o.into(),
            NakedCharacter::LowerP(lower_p) => lower_p.into(),
            NakedCharacter::LowerQ(lower_q) => lower_q.into(),
            NakedCharacter::LowerR(lower_r) => lower_r.into(),
            NakedCharacter::LowerS(lower_s) => lower_s.into(),
            NakedCharacter::LowerT(lower_t) => lower_t.into(),
            NakedCharacter::LowerU(lower_u) => lower_u.into(),
            NakedCharacter::LowerV(lower_v) => lower_v.into(),
            NakedCharacter::LowerW(lower_w) => lower_w.into(),
            NakedCharacter::LowerX(lower_x) => lower_x.into(),
            NakedCharacter::LowerY(lower_y) => lower_y.into(),
            NakedCharacter::LowerZ(lower_z) => lower_z.into(),
            NakedCharacter::UpperA(upper_a) => upper_a.into(),
            NakedCharacter::UpperB(upper_b) => upper_b.into(),
            NakedCharacter::UpperC(upper_c) => upper_c.into(),
            NakedCharacter::UpperD(upper_d) => upper_d.into(),
            NakedCharacter::UpperE(upper_e) => upper_e.into(),
            NakedCharacter::UpperF(upper_f) => upper_f.into(),
            NakedCharacter::UpperG(upper_g) => upper_g.into(),
            NakedCharacter::UpperH(upper_h) => upper_h.into(),
            NakedCharacter::UpperI(upper_i) => upper_i.into(),
            NakedCharacter::UpperJ(upper_j) => upper_j.into(),
            NakedCharacter::UpperK(upper_k) => upper_k.into(),
            NakedCharacter::UpperL(upper_l) => upper_l.into(),
            NakedCharacter::UpperM(upper_m) => upper_m.into(),
            NakedCharacter::UpperN(upper_n) => upper_n.into(),
            NakedCharacter::UpperO(upper_o) => upper_o.into(),
            NakedCharacter::UpperP(upper_p) => upper_p.into(),
            NakedCharacter::UpperQ(upper_q) => upper_q.into(),
            NakedCharacter::UpperR(upper_r) => upper_r.into(),
            NakedCharacter::UpperS(upper_s) => upper_s.into(),
            NakedCharacter::UpperT(upper_t) => upper_t.into(),
            NakedCharacter::UpperU(upper_u) => upper_u.into(),
            NakedCharacter::UpperV(upper_v) => upper_v.into(),
            NakedCharacter::UpperW(upper_w) => upper_w.into(),
            NakedCharacter::UpperX(upper_x) => upper_x.into(),
            NakedCharacter::UpperY(upper_y) => upper_y.into(),
            NakedCharacter::UpperZ(upper_z) => upper_z.into(),
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

impl From<Set> for character::Set {
    fn from(set: Set) -> Self {
        set.0.into_iter().map(Into::<Self>::into).sum()
    }
}

#[derive(Debug, Parser)]
pub struct Range(Element, Option<(Hyphen, Element)>);

impl From<Range> for character::Set {
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
    EscapedElement(Backslash, EscapedElement),
    NakedElement(NakedElement),
}

impl From<Element> for u8 {
    fn from(element: Element) -> Self {
        unimplemented!();
    }
}

impl From<Element> for character::Set {
    fn from(element: Element) -> Self {
        unimplemented!();
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
