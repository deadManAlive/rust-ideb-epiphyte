use std::{error::Error, io::{self, Write}};

use dll_syringe::{Syringe, process::OwnedProcess};

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(target_process) = OwnedProcess::find_first_by_name("run") {
        dbg!(&target_process);

        let syringe = Syringe::for_process(target_process);
        let injected_payload = syringe.inject("./target/i686-pc-windows-msvc/debug/epiphyte.dll")?;

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
    }
 
    Ok(())
}
