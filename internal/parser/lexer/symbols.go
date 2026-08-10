package lexer

import (
	. "github.com/fugalang/fugu/internal/parser/token"
)

func (lex *Lexer) readSymbol(ch rune) Token {
	switch ch {
	case '/':
		switch lex.rn {
		case '/':
			return lex.lineComment()
		case '*':
			return lex.multiLineComment()
		}
		return lex.token(lex.withAssign(DIV, DIV_ASSIGN))
	case '.':
		return lex.token(lex.readDot())
	case '<':
		return lex.token(lex.readLess())
	case '>':
		return lex.token(lex.readGreater())
	case '-':
		return lex.token(lex.readMinus())
	case '+':
		return lex.token(lex.withAssign(ADD, ADD_ASSIGN))
	case '*':
		return lex.token(lex.withAssign(MUL, MUL_ASSIGN))
	case '%':
		return lex.token(lex.withAssign(MOD, MOD_ASSIGN))
	case '^':
		return lex.token(lex.withAssign(POW, POW_ASSIGN))
	case '~':
		return lex.token(BITWISE_NOT)
	case '&':
		return lex.token(lex.withSuffix('&', REF, AND))
	case '$':
		return lex.token(lex.readInterpolationStart())
	case '!':
		return lex.token(lex.withSuffix('=', BANG, NEQ))
	case '?':
		return lex.token(lex.readQuestion())
	case '=':
		return lex.token(lex.readEqual())
	case '|':
		return lex.token(lex.readPipe())
	case ':':
		return lex.token(COLON)
	case '(':
		return lex.token(L_PAREN)
	case ')':
		return lex.token(R_PAREN)
	case '{':
		lex.openBrace()
		return lex.token(L_BRACE)
	case '}':
		lex.closeBrace()
		return lex.token(R_BRACE)
	case '[':
		return lex.token(L_BRACK)
	case ']':
		return lex.token(R_BRACK)
	case ';':
		return lex.token(END)
	case ',':
		return lex.token(COMMA)
	default:
		return lex.token(ILLEGAL)
	}
}

func (lex *Lexer) withAssign(plain, assigned Kind) Kind {
	return lex.withSuffix('=', plain, assigned)
}

func (lex *Lexer) withSuffix(suffix rune, plain, compound Kind) Kind {
	if lex.accept(suffix) {
		return compound
	}
	return plain
}

func (lex *Lexer) readDot() Kind {
	if !lex.accept('.') {
		return DOT
	}
	switch {
	case lex.accept('='):
		return RANGE_INCL
	case lex.accept('<'):
		return RANGE_HALF_OPEN
	case lex.accept('.'):
		return OP_ARRAY
	default:
		return OP_RANGE
	}
}

func (lex *Lexer) readLess() Kind {
	switch {
	case lex.accept('<'):
		return SHR_LESS
	case lex.accept('='):
		return LE
	case lex.accept('-'):
		return CHAN_SEND
	default:
		return LT
	}
}

func (lex *Lexer) readGreater() Kind {
	switch {
	case lex.accept('>'):
		return SHR_GREATER
	case lex.accept('='):
		return GE
	default:
		return GT
	}
}

func (lex *Lexer) readMinus() Kind {
	switch {
	case lex.accept('='):
		return SUB_ASSIGN
	case lex.accept('>'):
		return RTN_ARROW
	default:
		return SUB
	}
}

func (lex *Lexer) readQuestion() Kind {
	switch {
	case lex.accept(':'):
		return DEFAULT
	case lex.accept('.'):
		return OPTIONAL_DOT
	default:
		return QUESTION
	}
}

func (lex *Lexer) readEqual() Kind {
	switch {
	case lex.accept('='):
		return EQ
	case lex.accept('>'):
		return ARROW
	default:
		return ASSIGN
	}
}

func (lex *Lexer) readPipe() Kind {
	switch {
	case lex.accept('|'):
		return OR
	case lex.accept('>'):
		return PIPE
	default:
		return ILLEGAL
	}
}

func (lex *Lexer) readInterpolationStart() Kind {
	if !lex.accept('{') {
		return ILLEGAL
	}
	lex.openBrace()
	return L_BRACE
}

func (lex *Lexer) openBrace() {
	if n := len(lex.interpStack); n > 0 {
		lex.interpStack[n-1]++
	}
}

func (lex *Lexer) closeBrace() {
	n := len(lex.interpStack)
	if n == 0 {
		return
	}

	lex.interpStack[n-1]--
	if lex.interpStack[n-1] == 0 {
		lex.interpStack = lex.interpStack[:n-1]
		lex.inStringResume = true
	}
}
