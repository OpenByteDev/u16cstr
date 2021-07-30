use std::{error::Error, path::PathBuf, process::Command, str::FromStr};

#[test]
fn basic_in_another_crate() -> Result<(), Box<dyn Error>> {
    let crate_name = "test-crate";
    let crate_path = PathBuf::from_str(".\\tests")?.join(crate_name).canonicalize()?;

    Command::new("cargo")
        .arg("test")
        .current_dir(&crate_path)
        .spawn()?
        .wait()?;

    Ok(())
}
