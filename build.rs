#[cfg(not(feature = "bundled"))]
fn main() {}

#[cfg(feature = "bundled")]
fn main() -> color_eyre::Result<()> {
    bundled::build_itext()
}

#[cfg(feature = "bundled")]
mod bundled {
    use cfg_if::cfg_if;
    use color_eyre::eyre::Error;
    use color_eyre::Result;
    use std::env::var;
    use std::fs;
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    pub fn build_itext() -> Result<()> {
        println!("cargo:rerun-if-changed=bundle");

        let manifest_dir = PathBuf::from(var("CARGO_MANIFEST_DIR")?);
        run_gradle_command("shadowjar")?;

        let builddir = manifest_dir.join("bundle").join("build").join("libs");

        let outjar = fs::read_dir(builddir)?
            .into_iter()
            .find(|entry| {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => return false,
                };
                let fname = entry.file_name();
                let fname = fname.to_string_lossy();

                fname.starts_with("bundle") && fname.ends_with("all.jar")
            })
            .and_then(|x| x.ok())
            .ok_or(Error::msg("Could not find output jar"))?
            .path();

        let outdir = PathBuf::from(var("OUT_DIR")?);
        fs::copy(outjar, outdir.join("dependencies.jar"))?;

        run_gradle_command("clean")?;

        Ok(())
    }

    fn gradle_command_name() -> &'static str {
        cfg_if! {
            if #[cfg(unix)] {
                "./gradlew"
            } else if #[cfg(windows)] {
                "gradlew.bat"
            } else {
                compiler_error!("Platform not supported");
            }
        }
    }

    fn run_gradle_command(cmd: &str) -> Result<()> {
        let manifest_dir = PathBuf::from(var("CARGO_MANIFEST_DIR")?);
        let exec_dir = manifest_dir.join("bundle").canonicalize()?;
        let program = exec_dir.join(gradle_command_name());

        let output = Command::new(&program)
            .arg(cmd)
            .current_dir(&exec_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        if output.status.success() {
            Ok(())
        } else {
            let stdout = String::from_utf8(output.stdout)?;
            eprintln!("{stdout}");
            let stderr = String::from_utf8(output.stderr)?;
            eprintln!("{stderr}");

            Err(Error::msg("Building Java dependency failed"))
        }
    }
}
