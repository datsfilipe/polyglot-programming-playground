// lesson 1: in rust, everything is moved when I sign it
// example: if I pass a value to a function and the function don't return the value, it's gone
// forever
// that's why we can use references, to keep values

// lesson 2: I cannot have a mutable reference along with a not mutable reference. I also cannot
// have two thing mutating the same thing (it seems).

// lesson 3: using let to declare values allows us to change the value from one type to another.

fn main() {
    // let a = vec![];
    // let mut b = a;
    //
    // b.push(1);
    // it's not possible to:
    // a.push(1);

    // println!("{:?}", b);
    let mut x = 5;
    
    // that just lives in the scope (it seems to be not the final explanation but will serve for
    // now):
    {
        // let y = &x;
        // can't do that while having y:
        // let z = &mut x;
        // without y is totally fine:
        let z = &mut x;

        *z = 7;

        // we coudn't mutate x while mutating it's reference,
        // but if we scope the reference like this, outside the scope x can be mutated
    }

    // it works
    x = 9;

    println!("{:?}", x);
}
