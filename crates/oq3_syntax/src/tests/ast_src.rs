//! Defines input for code generation process.

pub(crate) struct KindsSrc<'a> {
    pub(crate) punct: &'a [(&'a str, &'a str)],
    pub(crate) keywords: &'a [&'a str],
    pub(crate) literals: &'a [&'a str],
    pub(crate) scalar_types: &'a [&'a str],
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [&'a str],
}

pub(crate) const KINDS_SRC: KindsSrc<'_> = KindsSrc {
    punct: &[
        ("++", "DOUBLE_PLUS"),
        (";", "SEMICOLON"),
        (",", "COMMA"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("@", "AT"),
        ("#", "POUND"),
        ("~", "TILDE"),
        ("?", "QUESTION"),
        ("$", "DOLLAR"),
        ("&", "AMP"),
        ("|", "PIPE"),
        ("+", "PLUS"),
        ("*", "STAR"),
        ("/", "SLASH"),
        ("^", "CARET"),
        ("%", "PERCENT"),
        ("_", "UNDERSCORE"),
        (".", "DOT"),
        ("..", "DOT2"),
        ("...", "DOT3"),
        ("..=", "DOT2EQ"),
        (":", "COLON"),
        ("::", "COLON2"),
        ("=", "EQ"),
        ("==", "EQ2"),
        ("=>", "FAT_ARROW"),
        ("!", "BANG"),
        ("!=", "NEQ"),
        ("-", "MINUS"),
        ("->", "THIN_ARROW"),
        ("<=", "LTEQ"),
        (">=", "GTEQ"),
        ("+=", "PLUSEQ"),
        ("-=", "MINUSEQ"),
        ("|=", "PIPEEQ"),
        ("&=", "AMPEQ"),
        ("^=", "CARETEQ"),
        ("/=", "SLASHEQ"),
        ("*=", "STAREQ"),
        ("%=", "PERCENTEQ"),
        ("&&", "AMP2"),
        ("||", "PIPE2"),
        ("<<", "SHL"),
        (">>", "SHR"),
        ("<<=", "SHLEQ"),
        (">>=", "SHREQ"),
    ],
    keywords: &[
        "OPENQASM", "include", "def", "defcalgrammar", "cal", "defcal", "gate",
        "delay", "reset", "measure",
        "pragma",  "end",
        "let",  "box", "extern",
        "const", "barrier",
        "gphase", // This is a slight hack because a `gphase` call has unique syntax.

        // Flow control
        "if", "else", "for", "in", "while", "continue", "return", "break",

        // Types
        "input", "output", "readonly", "mutable", "qreg", "creg",
        "qubit", "void", "array",

        // I suppose these are literals
        "false", "true",
    ],
    // GJL: try introducing scalar_types to help parse var declarations. May not be useful
    // sourcegen_ast.rs can convert these to upper snake case.
    scalar_types: &["float", "int", "uint", "complex", "bool", "bit", "duration", "stretch",  "angle"],
    // These are already upper snake case.
    literals: &["INT_NUMBER", "FLOAT_NUMBER", "SIMPLE_FLOAT_NUMBER", "CHAR", "BYTE", "STRING", "BIT_STRING", "TIMING_FLOAT_NUMBER", "TIMING_INT_NUMBER"],
    tokens: &["ERROR", "IDENT", "HARDWAREIDENT", "WHITESPACE", "COMMENT",], // FIXME, prob remove HARDWAREIDENT
    nodes: &[
        "SOURCE_FILE",
        "GATE",
        "DEF_CAL",
        "CAL",
        "DEF_CAL_GRAMMAR",
        "MEASURE",
        "BARRIER",
        "DEF",
        "RESET",
        "RET_TYPE",
        "CONST",
        "PAREN_TYPE",
        "PATH_TYPE",
        "SLICE_TYPE",
        // atoms
        "TUPLE_EXPR",
        "ARRAY_EXPR",
        "PAREN_EXPR",
        "PATH_EXPR",
        "IF_STMT",
        "WHILE_STMT",
        "FOR_STMT",
        "END_STMT",
        "CONTINUE_STMT",
        "BREAK_STMT",
        "LABEL",
        "BLOCK_EXPR",
        "STMT_LIST",
        "RETURN_EXPR",
//        "LET_EXPR",
        "LET_STMT",
        "ALIAS_EXPR",
        "CONCATENATION_EXPR",
        "BOX_EXPR",
        // postfix
        "CALL_EXPR",
        "CAST_EXPRESSION",
        "GATE_CALL_STMT",
        "G_PHASE_CALL_STMT",
        "INDEX_EXPR",
        // unary
        "PREFIX_EXPR",
        "RANGE_EXPR", // just weird
        "BIN_EXPR",
        "SET_NUM",
        "EXTERN_ITEM",
        "ITEM_LIST",
        "PATH",
        "PATH_SEGMENT",
        "LITERAL",
        "NAME",
        "NAME_REF",
        // "LET_ELSE",
        "EXPR_STMT",
        //        "TYPE_PARAM",
        "TYPE_SPEC", // "SPEC" to avoid the word "type"
        "TYPE_ARG",
        "TYPE",
        "NEW_TYPE",
        "DECLARED_VAR",
        "TYPE_DECLARATION_STMT",
        "RETURN_TYPE_ARG",
        "CONST_PARAM",
        "CONST_ARG",
        "PARAM_LIST",
        "QUBIT_LIST",
        "FILE_PATH",
        "PARAM",
        "ARG_LIST",
        "GATE_ARG_LIST",
        "VERSION",
        "VERSION_STRING",
        "INCLUDE",
        "DECLARATION",
        // From ANTLR grammar
        "DESIGNATOR",
        "SCALAR_TYPE",
        "SCALAR_TYPE_NAME",
        "ARRAY_TYPE",
        "QUBIT_TYPE",
        "EXPRESSION_LIST",
        "RETURN_SIGNATURE",
        "INT_NUM",
        "ALIAS_EXPRESSION",
        "SET_EXPRESSION",
        "ALIAS_DECLARATION_STATEMENT",
        "INDEX_OPERATOR",
        "INDEX_KIND",
        "INDEXED_IDENTIFIER",
        "IDENTIFIER",
//        "EXPR_OR_RANGE", // Remove if we dont take this route
        "ARRAY_LITERAL",
        "HARDWARE_QUBIT",
        "CLASSICAL_DECLARATION_STATEMENT",
        "ASSIGNMENT_STMT",
        "DECLARATION_EXPRESSION",
        "CONST_DECLARATION_STATEMENT",
        "I_O_DECLARATION_STATEMENT",
        "GATE_OPERAND",
        "MEASURE_EXPRESSION",
        "OLD_STYLE_DECLARATION_STATEMENT",
        "QUANTUM_DECLARATION_STATEMENT",
    ],
};

#[derive(Default, Debug)]
pub(crate) struct AstSrc {
    pub(crate) tokens: Vec<String>,
    pub(crate) nodes: Vec<AstNodeSrc>,
    pub(crate) enums: Vec<AstEnumSrc>,
}

#[derive(Debug)]
pub(crate) struct AstNodeSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
    Token(String),
    Node { name: String, ty: String, cardinality: Cardinality },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Cardinality {
    Optional,
    Many,
}

#[derive(Debug)]
pub(crate) struct AstEnumSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) variants: Vec<String>,
}