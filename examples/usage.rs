use ir_aquila::{stack_error, StackError};
use snafu::Snafu;

fn main() {
    let e = Error::Foo { source: Internal::Bar { msg: "msg".to_string() } };
    println!("{:?}", e);
    println!("{}", e.root_cause());
}

#[derive(Snafu)]
#[stack_error]
enum Error {
    #[snafu(display("Foo"))]
    Foo {
        source: Internal
    }
}

#[derive(Snafu)]
#[stack_error]
enum Internal {
    #[snafu(display("Bar, error: {}", msg))]
    Bar {
        msg: String
    }
}