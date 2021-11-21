pub mod docker;
pub mod light;
pub mod pipewire;
pub mod sound;

pub mod commands {
    use std::process::{Command, Output};
    const PRIVILLEGE_COMMAND: &str = "doas";

    pub struct TostCmd<'a> {
        cmd: &'a str,
        args: Vec<&'a str>,
        needs_notify: (bool, &'a str),
        needs_privilleges: bool,
    }

    impl<'a> TostCmd<'a> {
        pub fn new(cmd: &'a str, args: Vec<&'a str>) -> Self {
            TostCmd {
                cmd,
                args,
                needs_notify: (false, ""),
                needs_privilleges: false,
            }
        }

        pub fn add_privilleges(mut self) -> Self {
            self.needs_privilleges = true;
            self.args.insert(0, self.cmd);
            self.cmd = PRIVILLEGE_COMMAND;
            self
        }

        pub fn add_notify(mut self, signal: &'a str) -> Self {
            self.needs_notify = (true, signal);
            self
        }

        fn notify(&self) {
            let (needs, signal) = self.needs_notify;
            if needs {
                Command::new("pkill")
                    .arg(format!("-RTMIN+{}", signal))
                    .arg("dwmblocks")
                    .status()
                    .expect("Could not execute command");
            }
        }

        pub fn run(self) {
            Command::new(self.cmd)
                .args(&self.args)
                .status()
                .expect("Could not execute command");

            self.notify();
        }

        pub fn run_status(self) -> bool {
            let res = Command::new(self.cmd)
                .args(&self.args)
                .status()
                .expect("Could not execute command")
                .success();

            self.notify();

            res
        }

        pub fn run_output(self) -> Output {
            let res = Command::new(self.cmd)
                .args(&self.args)
                .output()
                .expect("Could not execute command");

            self.notify();

            res
        }
    }
}
