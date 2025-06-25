use std::{
    error::Error,
    io::{self, Write},
};

use dll_syringe::{Syringe, process::OwnedProcess};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let process_name = &args[1];
    let injected_dll = &args[2];

    dbg!(process_name);
    dbg!(injected_dll);

    if let Some(target_process) = OwnedProcess::find_first_by_name(process_name) {
        dbg!(&target_process);

        let syringe = Syringe::for_process(target_process);
        let injected_payload = syringe.inject(injected_dll)?;

        loop {
            print!(">> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let trimmed = input.trim();

            if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("bye") {
                break;
            }
        }

        println!("Exiting...");

        syringe.eject(injected_payload)?;
    } else {
        println!("The app doesn't seem to be run...");
    }

    Ok(())
}
