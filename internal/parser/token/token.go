package token

type Kind uint16

const (
	_         Kind = iota
	ILLEGAL        // unknown token
	COMMENT        // comment
	M_COMMENT      // /* comment */
	SPACING        // whitespace
	EOF

	// Groups
	G_NUMBER
	G_STRING
	G_LITERAL
	G_ARITHMETIC

	INTEGER    // 123
	IMAGINARY  // 123i
	FLOATING   // 12.3
	STRING     // "abc"
	RAW_STRING // `abc`
	CHARACTER  // 'a'
	IDENTIFIER // myVar

	// Module declaration and import keywords
	MODULE
	USE

	// Declaration keywords
	FN
	IMPL
	LET
	MUT
	CONST
	TYPE
	ENUM
	STRUCT
	INTERFACE

	// Control flow constructs
	RETURN
	MATCH
	SWITCH
	CASE
	IF
	ELSE
	FOR
	RANGE
	CONTINUE
	BREAK
	GOTO

	// Execution control

	DEFER  // deferred function call before leaving the current scope
	SELECT // wait for the first ready event from channels/coroutines

	// Grouping operators

	L_PAREN // (
	R_PAREN // )
	L_BRACE // {
	R_BRACE // }
	L_BRACK // [
	R_BRACK // ]

	// Assignment operators

	ASSIGN     // =
	SUB_ASSIGN // -=
	ADD_ASSIGN // +=
	MUL_ASSIGN // *=
	DIV_ASSIGN // /=
	MOD_ASSIGN // %=
	POW_ASSIGN // ^=

	// Comparison and logical operators

	EQ   // ==
	NEQ  // !=
	LE   // <=
	GE   // >=
	LT   // <
	GT   // >
	BANG // !
	AND  // &&
	OR   // ||

	// Arithmetic operators

	SUB // -
	ADD // +
	MUL // *
	DIV // /
	MOD // %
	POW // ^

	// Bit shift and bitwise operators

	SHR_LESS    // <<
	SHR_GREATER // >>
	BITWISE_NOT // ~

	// Range operators

	OP_RANGE        // ..   (exclusive / open)
	RANGE_INCL      // ..=  (inclusive / closed)
	RANGE_HALF_OPEN // ..<  (half-open)
	OP_ARRAY        // ...

	// Data flow operators

	ARROW        //  =>
	RTN_ARROW    // ->
	PIPE         // |>
	DEFAULT      // ?:
	QUESTION     // ?
	OPTIONAL_DOT // ?.
	REF          // &
	CHAN_SEND    // <-

	// Separator operators

	COLON // :
	END   // ;
	COMMA // ,
	DOT   // .

	EndToken
)

func (tk *Kind) Group() Kind {
	switch *tk {
	case INTEGER, IMAGINARY, FLOATING:
		return G_NUMBER
	case STRING, RAW_STRING, CHARACTER:
		return G_STRING
	case SUB, ADD, MUL, DIV, MOD, POW:
		return G_ARITHMETIC
	case G_ARITHMETIC, G_STRING, IDENTIFIER:
		return G_LITERAL
	default:
		return *tk
	}
}

func Expand(tk Kind) []Kind {
	switch tk {
	case G_LITERAL:
		return []Kind{
			G_NUMBER,
			G_STRING,
			INTEGER,
			IMAGINARY,
			FLOATING,
			STRING,
			RAW_STRING,
			CHARACTER,
			IDENTIFIER,
		}
	case G_NUMBER:
		return []Kind{
			INTEGER,
			IMAGINARY,
			FLOATING,
		}
	case G_STRING:
		return []Kind{
			STRING,
			RAW_STRING,
			CHARACTER,
		}
	case G_ARITHMETIC:
		return []Kind{
			SUB,
			ADD,
			MUL,
			DIV,
			MOD,
			POW,
		}
	default:
		return nil
	}
}

func Group(tk Kind) Kind {
	return tk.Group()
}

type Token struct {
	Kind  Kind
	Pos   Position // token start position
	Start uint64   // absolute offset to the token start
	End   uint64   // absolute offset to the token end
}

// Position describes a token location in source code.
type Position struct {
	FileName string
	Line     uint64
	Column   uint64
	Offset   uint64
}

func (tk Token) Literal(source *[]byte) []byte {
	b := (*source)[tk.Start:tk.End]

	switch tk.Kind {
	case STRING, RAW_STRING:
		if len(b) >= 1 && b[0] == '"' {
			if len(b) >= 2 && b[len(b)-1] == '"' {
				return b[1 : len(b)-1]
			}
			return b[1:]
		}

		if len(b) >= 1 && b[0] == '`' {
			if len(b) >= 2 && b[len(b)-1] == '`' {
				return b[1 : len(b)-1]
			}
			return b[1:]
		}

		return b

	case CHARACTER:
		if len(b) >= 2 && b[0] == '\'' && b[len(b)-1] == '\'' {
			return b[1 : len(b)-1]
		}
		return b

	case COMMENT:
		if len(b) >= 2 && b[0] == '/' && b[1] == '/' {
			return b[2:]
		}
		return b

	case M_COMMENT:
		if len(b) >= 4 &&
			b[0] == '/' && b[1] == '*' &&
			b[len(b)-2] == '*' && b[len(b)-1] == '/' {
			return b[2 : len(b)-2]
		}
		return b

	default:
		return b
	}
}

var keywords = map[string]Kind{
	// Module declaration and import
	"module": MODULE,
	"use":    USE,

	// Data type and variable declarations
	"fn":        FN,
	"impl":      IMPL,
	"let":       LET,
	"mut":       MUT,
	"const":     CONST,
	"type":      TYPE,
	"enum":      ENUM,
	"struct":    STRUCT,
	"interface": INTERFACE,

	// Control flow constructs
	"return":   RETURN,
	"match":    MATCH,
	"if":       IF,
	"else":     ELSE,
	"switch":   SWITCH,
	"case":     CASE,
	"for":      FOR,
	"range":    RANGE,
	"continue": CONTINUE,
	"break":    BREAK,
	"goto":     GOTO,

	// Execution control
	"defer":  DEFER,
	"select": SELECT,
}

// SearchKeyword checks whether ident is a reserved keyword.
// It returns the keyword kind or IDENTIFIER otherwise.
func SearchKeyword(ident []byte) Kind {
	if kind, ok := keywords[string(ident)]; ok {
		return kind
	}
	return IDENTIFIER
}

func (tk Kind) String() string {
	switch tk {
	case ILLEGAL:
		return "ILLEGAL"
	case COMMENT:
		return "COMMENT"
	case SPACING:
		return "SPACING"
	case EOF:
		return "EOF"
	case INTEGER:
		return "INTEGER"
	case IMAGINARY:
		return "IMAGINARY"
	case FLOATING:
		return "FLOATING"
	case STRING:
		return "STRING"
	case RAW_STRING:
		return "RAW_STRING"
	case CHARACTER:
		return "CHARACTER"
	case IDENTIFIER:
		return "IDENTIFIER"
	case MODULE:
		return "MODULE"
	case USE:
		return "USE"
	case FN:
		return "FN"
	case IMPL:
		return "IMPL"
	case LET:
		return "LET"
	case MUT:
		return "MUT"
	case CONST:
		return "CONST"
	case TYPE:
		return "TYPE"
	case ENUM:
		return "ENUM"
	case STRUCT:
		return "STRUCT"
	case INTERFACE:
		return "INTERFACE"
	case RETURN:
		return "RETURN"
	case MATCH:
		return "MATCH"
	case IF:
		return "IF"
	case ELSE:
		return "ELSE"
	case SWITCH:
		return "SWITCH"
	case CASE:
		return "CASE"
	case FOR:
		return "FOR"
	case RANGE:
		return "RANGE"
	case GOTO:
		return "GOTO"
	case CONTINUE:
		return "CONTINUE"
	case BREAK:
		return "BREAK"
	case DEFER:
		return "DEFER"
	case SELECT:
		return "SELECT"
	case ASSIGN:
		return "ASSIGN"
	case SUB_ASSIGN:
		return "SUB_ASSIGN"
	case ADD_ASSIGN:
		return "ADD_ASSIGN"
	case MUL_ASSIGN:
		return "MUL_ASSIGN"
	case DIV_ASSIGN:
		return "DIV_ASSIGN"
	case MOD_ASSIGN:
		return "MOD_ASSIGN"
	case EQ:
		return "EQ"
	case NEQ:
		return "NEQ"
	case LE:
		return "LE"
	case GE:
		return "GE"
	case LT:
		return "LT"
	case GT:
		return "GT"
	case BANG:
		return "BANG"
	case AND:
		return "AND"
	case OR:
		return "OR"
	case SUB:
		return "SUB"
	case ADD:
		return "ADD"
	case MUL:
		return "MUL"
	case DIV:
		return "DIV"
	case MOD:
		return "MOD"
	case POW:
		return "POW"
	case SHR_LESS:
		return "SHR_LESS"
	case SHR_GREATER:
		return "SHR_GREATER"
	case BITWISE_NOT:
		return "BITWISE_NOT"
	case REF:
		return "REF"
	case OP_RANGE:
		return "OP_RANGE"
	case RANGE_HALF_OPEN:
		return "RANGE_HALF_OPEN"
	case RANGE_INCL:
		return "RANGE_INCL"
	case ARROW:
		return "ARROW"
	case PIPE:
		return "PIPE"
	case DEFAULT:
		return "DEFAULT"
	case QUESTION:
		return "QUESTION"
	case OPTIONAL_DOT:
		return "OPTIONAL_DOT"
	case L_PAREN:
		return "L_PAREN"
	case R_PAREN:
		return "R_PAREN"
	case L_BRACE:
		return "L_BRACE"
	case R_BRACE:
		return "R_BRACE"
	case L_BRACK:
		return "L_BRACK"
	case R_BRACK:
		return "R_BRACK"
	case COLON:
		return "COLON"
	case END:
		return "END"
	case COMMA:
		return "COMMA"
	case DOT:
		return "DOT"
	}
	return ""
}
