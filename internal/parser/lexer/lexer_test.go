package lexer

import (
	"reflect"
	"testing"

	. "github.com/fugalang/fugu/internal/parser/token"
)

func TestOperators(t *testing.T) {
	source := []byte("/ /= . .. ..= ..< ... < << <= <- > >> >= - -= -> + += * *= % %= ^ ^= ~ & && $ ! != ? ?: ?. = == => | || |> : ( ) { } [ ] ; ,")
	want := []Kind{
		DIV, DIV_ASSIGN, DOT, OP_RANGE, RANGE_INCL, RANGE_HALF_OPEN, OP_ARRAY,
		LT, SHR_LESS, LE, CHAN_SEND, GT, SHR_GREATER, GE,
		SUB, SUB_ASSIGN, RTN_ARROW, ADD, ADD_ASSIGN, MUL, MUL_ASSIGN,
		MOD, MOD_ASSIGN, POW, POW_ASSIGN, BITWISE_NOT, REF, AND,
		ILLEGAL, BANG, NEQ, ILLEGAL, DEFAULT, OPTIONAL_DOT,
		ASSIGN, EQ, ARROW, ILLEGAL, OR, PIPE, COLON,
		L_PAREN, R_PAREN, L_BRACE, R_BRACE, L_BRACK, R_BRACK, END, COMMA,
	}

	if got := tokenKinds(source, false); !reflect.DeepEqual(got, want) {
		t.Fatalf("token kinds mismatch\n got: %v\nwant: %v", got, want)
	}
}

func TestLiteralsAndKeywords(t *testing.T) {
	source := []byte("fn name 123 1.5 2i 12abc \"text\" `raw` 'x' // note\n/* block */")
	want := []Kind{
		FN, IDENTIFIER, INTEGER, FLOATING, IMAGINARY, IDENTIFIER,
		STRING, RAW_STRING, CHARACTER, COMMENT, M_COMMENT,
	}

	if got := tokenKinds(source, false); !reflect.DeepEqual(got, want) {
		t.Fatalf("token kinds mismatch\n got: %v\nwant: %v", got, want)
	}
}

func TestInterpolation(t *testing.T) {
	source := []byte("\"hello ${name}\"")
	want := []Kind{STRING, L_BRACE, IDENTIFIER, R_BRACE, STRING}

	if got := tokenKinds(source, false); !reflect.DeepEqual(got, want) {
		t.Fatalf("token kinds mismatch\n got: %v\nwant: %v", got, want)
	}
}

func TestSpacingPosition(t *testing.T) {
	lex := New([]byte("x \n y"), "test.fg")
	_ = lex.NextToken()
	spacing := lex.NextToken()

	if spacing.Kind != SPACING {
		t.Fatalf("kind = %v, want %v", spacing.Kind, SPACING)
	}
	if spacing.Pos.Offset != 1 || spacing.Pos.Line != 1 || spacing.Pos.Column != 2 {
		t.Fatalf("position = %+v, want offset 1, line 1, column 2", spacing.Pos)
	}
}

func TestChannelSendIncludesSuffix(t *testing.T) {
	source := []byte("<-")
	token := New(source, "").NextToken()

	if token.Kind != CHAN_SEND {
		t.Fatalf("kind = %v, want %v", token.Kind, CHAN_SEND)
	}
	if got := string(token.Literal(&source)); got != "<-" {
		t.Fatalf("literal = %q, want %q", got, "<-")
	}
}

func tokenKinds(source []byte, includeSpacing bool) []Kind {
	lex := New(source, "")
	var kinds []Kind
	for {
		token := lex.NextToken()
		if token.Kind == EOF {
			return kinds
		}
		if includeSpacing || token.Kind != SPACING {
			kinds = append(kinds, token.Kind)
		}
	}
}
