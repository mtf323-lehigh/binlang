use binlang::{compile, run};

fn main() {
    let src = "{1,0,0,1,0,1}; 1; {:4}; let x = 1; let y = {1,0,1,1};";
    let prg = match compile(src) {
        Ok(p) => p,
        Err(_) => {
            println!("There was a problem compiling the program.");
            return;
        }
    };
    run(prg).unwrap();
}
