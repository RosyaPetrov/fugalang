package lexer

import (
	. "github.com/fugalang/fugu/internal/parser/token"
)

func (lex *Lexer) recoverUnterminated() Token {
	token := lex.token(ILLEGAL)
	lex.unfreeze()
	lex.stabilize()
	return token
}

func (lex *Lexer) stabilize() {
	for {
		lex.freeze()
		token := lex.NextToken()
		switch token.Kind {
		case EOF:
			return
		case SPACING, COMMENT, M_COMMENT:
			continue
		case FN, IF, ELSE, SWITCH, CASE, RETURN, ENUM, SELECT, R_BRACE, END:
			lex.unfreeze()
			return
		}
	}
}
