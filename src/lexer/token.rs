pub struct Token {
    pub kind: TokenKind,
    pub category: TokenCategory,
    pub value: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, value: Option<String>, category: TokenCategory) -> Token {
        Token {
            kind,
            value,
            category,
        }
    }
}


pub enum TokenCategory {
    Keyword,
    Dunder,
    BuiltInType,
    BuiltInFn,
    PunctuationAndGroup,
    Operators,
    Comparison,
    Literal,
    Identifier,
    Whitespace,
    Eof,
    Comment,
}

#[derive(PartialEq)]
pub enum TokenKind {
    Indent,
    Dedent,

    // Comments
    StringMultiline, // it can be both comment and string so it cannot be ignored
    CommentSingleline,

    // Standard keywords
    False,
    None,
    True,
    And,
    As,
    Assert,
    Async,
    Await,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    Local,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    Try,
    While,
    With,
    Yield,

    // Special methods (commonly used, but not reserved keywords)
    Init,    // __init__
    New,     // __new__
    Delitem, // __delitem__
    Getitem, // __getitem__
    Setitem, // __setitem__
    Str,     // __str__
    Repr,    // __repr__
    Len,     // __len__
    // Special cases
    Name,        // __name__
    Doc,         // __doc__
    Package,     // __package__
    Loader,      // __loader__
    Spec,        // __spec__
    Annotations, // __annotations__
    Builtins,    // __builtins__
    ImportFn,    // __import__

    // Built-in types
    Int,
    Float,
    Complex,
    List,
    Tuple,
    RangeType,
    String,
    Set,
    Dict,
    FrozenSet,
    ByteArray,
    Bytes,
    MemoryView,
    Bool,

    // Built-in functions
    Abs,
    All,
    Any,
    Ascii,
    Bin,
    BoolFn,
    Breakpoint,
    Bytearray,
    BytesFn,
    Callable,
    Chr,
    Classmethod,
    Compile,
    ComplexFn,
    Delattr,
    DictFn,
    Dir,
    Divmod,
    Enumerate,
    Eval,
    Exec,
    Filter,
    FloatFn,
    Format,
    Frozenset,
    Getattr,
    Globals,
    Hasattr,
    Hash,
    Help,
    Hex,
    Id,
    Input,
    IntFn,
    Isinstance,
    Issubclass,
    Iter,
    LenFn,
    ListFn,
    Locals,
    Map,
    Max,
    Memoryview,
    Min,
    Next,
    Object,
    Oct,
    Open,
    Ord,
    Pow,
    Print,
    Property,
    Range,
    ReprFn,
    Reversed,
    Round,
    SetFn,
    Setattr,
    Slice,
    Sorted,
    Staticmethod,
    StrFn,
    Sum,
    Super,
    TupleFn,
    Type,
    Vars,
    Zip,

    // Punctuation and grouping
    LeftParenthesis,  // (
    RightParenthesis, // )
    LeftBracket,      // [
    RightBracket,     // ]
    LeftBrace,        // {
    RightBrace,       // }
    Comma,            // ,
    Dot,              // .
    Semicolon,        // ;
    Colon,            // :
    Arrow,            // ->
    Ellipsis,         // ...

    // Operators
    Plus,             // +
    Minus,            // -
    Multiply,         // *
    Divide,           // /
    Modulo,           // %
    Power,            // **
    FloorDivide,      // //
    Increment,        // ++
    Decrement,        // --
    PlusEqual,        // +=
    MinusEqual,       // -=
    MultiplyEqual,    // *=
    DivideEqual,      // /=
    ModuloEqual,      // %=
    PowerEqual,       // **=
    FloorDivideEqual, // //=
    AndEqual,         // &=
    OrEqual,          // |=
    XorEqual,         // ^=
    ShiftLeftEqual,   // <<=
    ShiftRightEqual,  // >>=
    Assign,           // =

    // Comparison and logical operators
    Equal,        // ==
    NotEqual,     // !=
    Greater,      // >
    Less,         // <
    GreaterEqual, // >=
    LessEqual,    // <=
    AndCmp,       // &&
    OrCmp,        // ||
    NotCmp,       // !
    BitwiseAnd,   // &
    BitwiseOr,    // |
    BitwiseXor,   // ^
    BitwiseNot,   // ~
    ShiftLeft,    // <<
    ShiftRight,   // >>

    Eof,
    Ident,
    Newline,
    Whitespace,
}