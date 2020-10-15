package main

//#include <stdlib.h>
import "C"

import (
	"encoding/json"
	"fmt"
)

type Doggo struct {
	Name        string
	Age         int32
	Friend      string
	Friend_Name string
	Friend_Age  int32
}

//export cgo_ptr_char
func cgo_ptr_char() *C.char {
	Doggo := Doggo{"Doggo 👀", 10, "Crab", "Ferris", 9}
	hello_world, err := json.Marshal(Doggo)
	if err != nil {
		fmt.Printf("Error: %s", err)
		return C.CString("GOError4044")
	}
	return C.CString(string(hello_world))
}

func main() {}
