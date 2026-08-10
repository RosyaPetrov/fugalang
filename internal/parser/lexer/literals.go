package lexer

import (
	. "github.com/fugalang/fugu/internal/parser/token"
)

func (lex *Lexer) lineComment() Token {
	lex.advance()
	for lex.rn != '\n' && lex.rn != 0 {
		lex.advance()
	}
	return lex.token(COMMENT)
}

func (lex *Lexer) multiLineComment() Token {
	lex.advance()
	lex.freeze()
	for {
		switch {
		case lex.rn == 0:
			return lex.recoverUnterminated()
		case lex.rn == '*' && lex.peek() == '/':
			lex.advance()
			lex.advance()
			return lex.token(M_COMMENT)
		default:
			lex.advance()
		}
	}
}

func (lex *Lexer) readString() Token {
	lex.freeze()
	for lex.rn != '"' && lex.rn != 0 {
		switch {
		case lex.rn == '\\':
			lex.advance()
			if lex.rn != 0 {
				lex.advance()
			}
		case lex.rn == '$' && lex.peek() == '{':
			lex.interpStack = append(lex.interpStack, 0)
			return lex.token(STRING)
		default:
			lex.advance()
		}
	}

	if lex.rn == 0 {
		return lex.recoverUnterminated()
	}
	lex.advance()
	return lex.token(STRING)
}

func (lex *Lexer) readRawString() Token {
	lex.freeze()
	for lex.rn != '`' && lex.rn != 0 {
		lex.advance()
	}
	if lex.rn == 0 {
		return lex.recoverUnterminated()
	}
	lex.advance()
	return lex.token(RAW_STRING)
}

func (lex *Lexer) readChar() Token {
	if lex.rn == '\\' {
		lex.advance()
		if lex.rn != 0 {
			lex.advance()
		}
	} else if lex.rn != '\'' && lex.rn != 0 {
		lex.advance()
	}
	if !lex.accept('\'') {
		return lex.token(ILLEGAL)
	}
	return lex.token(CHARACTER)
}

func (lex *Lexer) readIdent() Token {
	for isIdentContinue(lex.rn) {
		lex.advance()
	}
	literal := lex.Input[lex.tokStart.offset:lex.pos]
	return lex.token(SearchKeyword(literal))
}

func (lex *Lexer) readNumber() Token {
	isFloat := false
	isIdent := false

	for {
		switch {
		case isASCIIDigit(lex.rn):
			lex.advance()
		case lex.rn == '.' && !isIdent && !isFloat && isASCIIDigit(lex.peek()):
			isFloat = true
			lex.advance()
		case isIdentContinue(lex.rn):
			isIdent = true
			lex.advance()
		default:
			return lex.token(lex.numberKind(isFloat, isIdent))
		}
	}
}

func (lex *Lexer) numberKind(isFloat, isIdent bool) Kind {
	if !isIdent {
		if isFloat {
			return FLOATING
		}
		return INTEGER
	}

	literal := lex.Input[lex.tokStart.offset:lex.pos]
	if isImaginary(literal) {
		return IMAGINARY
	}
	return IDENTIFIER
}

func isImaginary(literal []byte) bool {
	if len(literal) < 2 || literal[len(literal)-1] != 'i' {
		return false
	}
	for _, ch := range literal[:len(literal)-1] {
		if !isASCIIDigit(rune(ch)) && ch != '.' {
			return false
		}
	}
	return true
}
