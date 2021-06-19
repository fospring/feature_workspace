use argh::FromArgs;
use std::{
    env,
    process::{Command, Stdio},
};

/// fospring build tools
#[derive(FromArgs, Debug)]
struct Options {
    #[argh(subcommand)]
    command: Cmd
}
#[derive(FromArgs, Debug)]
#[argh(subcommand)]
/// Reach new heights.
enum Cmd {
    Build(BuildCmd)
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();
    match options.command {
        Cmd::Build(cmd) => cmd.build()
    }
}

/// build cmd
#[derive(FromArgs, Debug, Default)]
#[argh(subcommand, name = "build")]
struct BuildCmd {}
impl BuildCmd {
    fn build(&self) -> anyhow::Result<()> {
        let mut cmd = Command::new("cargo");
        let pwd = env::current_dir()?;
        cmd.current_dir(&pwd)
            .stderr(Stdio::inherit())
            .stdout(Stdio::piped())
            .arg("rustc")
            .arg("--manifest-path")
            .arg(pwd.join("fospring").join("Cargo.toml"));
        let output = cmd.output()?;
        if !output.status.success() {
            anyhow::bail!("build project failed");
        }
        Ok(())
    }
}