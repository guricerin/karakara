extern crate karakara;

fn main() {
    println!("karakara -is toy shell: \n");
    karakara::karakara::repl();
    println!("goodbye.\n");
}