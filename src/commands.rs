use std::process::{Command, Output};

pub struct TostCmd<'a> {
    cmd: &'a str,
    args: &'a [&'a str],
    needs_notify: Option<&'a str>,
}

impl<'a> TostCmd<'a> {
    pub fn new(cmd: &'a str, args: &'a [&'a str]) -> Self {
        TostCmd {
            cmd,
            args,
            needs_notify: None,
        }
    }

    pub fn add_notify(mut self, signal: &'a str) -> Self {
        self.needs_notify = Some(signal);
        self
    }

    pub fn run(self) {
        Command::new(self.cmd)
            .args(self.args)
            .status()
            .expect("to execute command");

        self.notify();
    }

    pub fn run_status(self) -> bool {
        let res = Command::new(self.cmd)
            .args(self.args)
            .status()
            .expect("to execute command")
            .success();

        self.notify();

        res
    }

    pub fn run_output(self) -> Output {
        let res = Command::new(self.cmd)
            .args(self.args)
            .output()
            .expect("to execute command");

        self.notify();

        res
    }

    fn notify(&self) {
        if let Some(signal) = self.needs_notify {
            Command::new("pkill")
                .arg(format!("-RTMIN+{}", signal))
                .arg("dwmblocks")
                .status()
                .expect("to execute command");
        }
    }
}
