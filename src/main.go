package main
import "fmt"

func returnsErr(value int) error {
  return fmt.Errorf("error %v", value)
}

type Foo struct { }

func (f *Foo) returnsErr(value int) error {
  return fmt.Errorf("error %v", value)
}

func CreateFoo(fail bool) (*Foo, error) {
  if fail {
    return nil, fmt.Errorf("error")
  }

  return &Foo{}, nil
}

func main() {
  foo, err := CreateFoo(false)
  if err != nil {
    panic(err)
  }

  foo.returnsErr(1)
}
