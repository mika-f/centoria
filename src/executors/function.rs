use std::process::{Command, ExitStatus};

use clap::ArgMatches;

use crate::executors::Executor;
use crate::formatter;

/**
 * function works as shell functions
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shell: Option<String>,
}

impl Function {
    pub fn new(
        command: &str,
        condition: Option<&str>,
        description: Option<&str>,
        shell: Option<&str>,
    ) -> Function {
        let condition = condition.map(|s| s.to_owned());
        let description = description.map(|s| s.to_owned());
        let shell = shell.map(|s| s.to_owned());

        return Function {
            command: command.to_owned(),
            condition,
            description,
            shell,
        };
    }

    fn shell(&self) -> &str {
        return match &self.shell {
            Some(shell) => &shell,
            None => "sh",
        };
    }
}

#[typetag::serde(name = "function")]
impl Executor for Function {
    fn can_execute(&self) -> bool {
        if self.shell() != "sh" {
            match Command::new(self.shell()).arg("--version").output() {
                Ok(_) => {}
                Err(_) => return false,
            };
        }

        if let Some(condition) = &self.condition {
            #[rustfmt::skip]
            return match Command::new(self.shell()).args(&["-c", &condition]).output() {
                Ok(value) => value.status.success(),
                Err(_) => false
            };
        }

        return true;
    }

    fn execute(&self, args: &ArgMatches) -> Result<ExitStatus, failure::Error> {
        let extra: Option<Vec<&str>> = args.values_of("extra").map(|w| w.collect());

        // checks
        let extra = match extra {
            Some(value) => value,
            None => {
                let msg = failure::err_msg("this function requires extra arguments");
                return Err(msg);
            }
        };

        // building
        let execute = match formatter::format_array(&self.command.to_string(), "", &extra) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };

        #[rustfmt::skip]
        match Command::new(self.shell()).args(&["-c", &execute.trim()]).status() {
            Ok(status) => return Ok(status),
            Err(e) => {
                let msg = failure::err_msg(format!("function failed: {}", e));
                return Err(msg);
            }
        };
    }
}
