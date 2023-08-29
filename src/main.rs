#![allow(dead_code)]
#![allow(unused_variables)]

trait MyTrait {
    fn do_something(&self);
}

#[derive(Debug)]
struct HasRefData<'a> {
    data: &'a [u8],
}
impl <'a> MyTrait for HasRefData<'a> {
    fn do_something(&self) {
        println!("in hasrefdata");
    }
}

#[derive(Debug)]
struct PlainStruct {
    intvalue: i32,
    stringvalue: String,
}

impl MyTrait for PlainStruct {
    fn do_something(&self) {
        println!("in plainstruct");
    }
}

#[derive(Debug)]
enum MyEnum<'a> {
    Tricky(HasRefData<'a>),
    Simple(PlainStruct),
}

impl <'a> MyTrait for MyEnum<'a> {
    fn do_something(&self) {
     
        match self {
            MyEnum::Tricky(v) => v.do_something(),
            MyEnum::Simple(v) => v.do_something(),
        }
    }
}

fn main() {
    let escaped = {
        let data = vec![1u8, 2u8, 3u8, 4u8];
        let oneenum = MyEnum::Tricky(HasRefData { data: &data });
        let anotherenum = MyEnum::Simple(PlainStruct {
            intvalue: 23,
            stringvalue: "John".to_string(),
        });
        let boxedupone : Box<dyn MyTrait> = Box::new(oneenum);
        let boxedupanother : Box<dyn MyTrait> = Box::new(anotherenum);
        boxedupone
    };

    escaped.do_something();
    // match escaped {
    //     MyEnum::Tricky(t) => {
    //         println!("tricky {:?}", t);
    //     }
    //     MyEnum::Simple(s) => {
    //         println!("simple {:?}", s);
    //     }
    // }
}
