package lexer

func isSpace(r rune) bool {
	switch r {
	case ' ', '\t', '\n', '\r', '\v', '\f':
		return true
	case 0x00A0, 0x1680,
		0x2000, 0x2001, 0x2002, 0x2003,
		0x2004, 0x2005, 0x2006, 0x2007,
		0x2008, 0x2009, 0x200A,
		0x2028, 0x2029, 0x202F, 0x205F,
		0x3000, 0xFEFF:
		return true
	default:
		return false
	}
}

func isASCIIDigit(r rune) bool {
	return r >= '0' && r <= '9'
}

func isIdentStart(r rune) bool {
	return r == '_' ||
		(r >= 'a' && r <= 'z') ||
		(r >= 'A' && r <= 'Z') ||
		r >= 0xAA
}

func isIdentContinue(r rune) bool {
	return isIdentStart(r) || isASCIIDigit(r) || r >= 0x0660
}
