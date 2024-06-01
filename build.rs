use std::fs;
use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
  println!("cargo::rerun-if-changed=resources/");
  for e in fs::read_dir("resources/ui")? {
    let path = e?.path();
    if path.extension().is_some_and(|e| e == "blp") {
      Command::new("blueprint-compiler")
        .arg("compile")
        .arg("--output")
        .arg(path.with_extension("ui"))
        .arg(path)
        .status()?;
    }
  }

  glib_build_tools::compile_resources(
    &["resources/"],
    "resources/resources.xml",
    "resources.gresource",
  );
  Ok(())
}
