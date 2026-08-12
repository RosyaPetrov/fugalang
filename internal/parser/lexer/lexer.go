package lexer

import (
	. "github.com/fugalang/fugu/internal/parser/token"
)

type cursorState struct {
	pos    int
	curPos int
	rn     rune
	rnSize int
	line   int
	column int
}

type sourcePosition struct {
	offset int
	line   int
	column int
}

type Lexer struct {
	Input    []byte
	fileName string

	cursorState
	saved    cursorState
	tokStart sourcePosition

	interpStack    []int
	inStringResume bool
}

func New(input []byte, fileName string) *Lexer {
	lex := &Lexer{
		Input:    input,
		fileName: fileName,
		cursorState: cursorState{
			line: 1,
		},
	}
	lex.advance()
	return lex
}

func (lex *Lexer) NextToken() Token {
	lex.markTokenStart()

	if isSpace(lex.rn) {
		for isSpace(lex.rn) {
			lex.advance()
		}
		return lex.token(SPACING)
	}

	if lex.rn == 0 {
		return lex.token(EOF)
	}

	if lex.inStringResume {
		lex.inStringResume = false
		return lex.readString()
	}

	ch := lex.rn
	lex.advance()

	switch {
	case ch == '"':
		return lex.readString()
	case ch == '`':
		return lex.readRawString()
	case ch == '\'':
		return lex.readChar()
	case isASCIIDigit(ch):
		return lex.readNumber()
	case isIdentStart(ch):
		return lex.readIdent()
	default:
		return lex.readSymbol(ch)
	}
}

func (lex *Lexer) markTokenStart() {
	lex.tokStart = sourcePosition{
		offset: lex.pos,
		line:   lex.line,
		column: lex.column,
	}
}

func (lex *Lexer) token(kind Kind) Token {
	return Token{
		Kind: kind,
		Pos: Position{
			FileName: lex.fileName,
			Line:     uint64(lex.tokStart.line),
			Column:   uint64(lex.tokStart.column),
			Offset:   uint64(lex.tokStart.offset),
		},
		Start: uint64(lex.tokStart.offset),
		End:   uint64(lex.pos),
	}
}
