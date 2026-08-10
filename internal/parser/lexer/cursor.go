package lexer

import "unicode/utf8"

func (lex *Lexer) advance() {
	if lex.curPos >= len(lex.Input) {
		lex.pos = lex.curPos
		lex.rn = 0
		lex.rnSize = 0
		return
	}

	lex.pos = lex.curPos
	lex.rn, lex.rnSize = decodeRune(lex.Input[lex.curPos:])
	lex.curPos += lex.rnSize

	if lex.rn == '\n' {
		lex.line++
		lex.column = 0
	} else {
		lex.column++
	}
}

func (lex *Lexer) peek() rune {
	if lex.curPos >= len(lex.Input) {
		return 0
	}
	r, _ := decodeRune(lex.Input[lex.curPos:])
	return r
}

func decodeRune(input []byte) (rune, int) {
	if input[0] < utf8.RuneSelf {
		return rune(input[0]), 1
	}
	return utf8.DecodeRune(input)
}

func (lex *Lexer) accept(want rune) bool {
	if lex.rn != want {
		return false
	}
	lex.advance()
	return true
}

func (lex *Lexer) freeze() {
	lex.saved = lex.cursorState
}

func (lex *Lexer) unfreeze() {
	lex.cursorState = lex.saved
}
