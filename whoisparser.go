package main

import "C"
import (
	whoisparser "github.com/likexian/whois-parser"
)

//export WhoIsParser
func WhoIsParser(raw string) (bool, bool, *C.char, *C.char) {
	result, err := whoisparser.Parse(raw)

	if err != nil {
		if err == whoisparser.ErrNotFoundDomain {
			return false, true, C.CString(""), C.CString("")
		}

		if err == whoisparser.ErrDomainDataInvalid || err == whoisparser.ErrDomainLimitExceed {
			return false, false, C.CString(""), C.CString("")
		}
	}

	var registrar string
	if result.Registrar == nil {
		registrar = ""
	} else {
		registrar = result.Registrar.Name
	}

	return true, true, C.CString(registrar), C.CString(result.Domain.ExpirationDate)
}

func main() {}
