
pub struct TopLevel<'a>(pub Vec<Message<'a>>);

/// message = { wcpad* ~ symbol ~ scpad* ~ arguments* }
pub struct Message<'a> {
    pub symbol: Symbol<'a>,
    pub arguments: Args<'a>,
}

pub type Args<'a> = Vec<Argument<'a>>;

/// argument = { wcpad* ~expression ~ wcpad* }
pub type Argument<'a> = Message<'a>;

/// symbol = { number | Operator | quote | Identifier }
pub enum Symbol<'a> {
    Number(&'a str),
    Operator(&'a str),
    Quote(&'a str),
    Identifier(&'a str),
}
