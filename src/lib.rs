pub mod docker;
pub mod light;
pub mod pipewire;
pub mod sound;

pub mod commands {
    use std::ffi::OsStr;
    use std::process::Command;

    pub fn run_cmd_notify<I, S>(cmd: &str, args: I, signal: &str)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        run_cmd(cmd, args);
        run_cmd(
            "pkill",
            [format!("-RTMIN+{}", signal), "dwmblocks".to_owned()],
        );
    }

    pub fn run_cmd<I, S>(cmd: &str, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        Command::new(cmd)
            .args(args)
            .status()
            .expect("Could not execute command")
            .success();
    }
}
