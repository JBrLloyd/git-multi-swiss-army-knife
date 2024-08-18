// use clap::Command;

// pub struct Subcommand<T> {
//     args: T
// }

pub trait SubcommandHandler {
    fn handle(&self) -> ();
}
