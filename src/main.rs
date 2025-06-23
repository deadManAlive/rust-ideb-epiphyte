use std::error::Error;

use dll_syringe::{Syringe, process::OwnedProcess};

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(target_process) = OwnedProcess::find_first_by_name("run") {
        dbg!(&target_process);

        let syringe = Syringe::for_process(target_process);
        let _ = syringe.inject("./target/i686-pc-windows-msvc/debug/epiphyte.dll")?;
    }

    Ok(())
}
