use rhai::{self, Scope};

use crate::SETTING;
use super::RHAI_INSTANCE;

pub fn ext_apply(body: &str) -> String {
    let lines: Vec<String> = body.lines().map(|line| line.to_string()).collect();
    let mut lines_output: Vec<String> = Vec::new();

    for line in &lines {
        if line.starts_with("!!") && SETTING.rhai_ext_command {
            let command_line: String = line.chars().skip(2).collect();

            let command_and_args_ = command_line.splitn(2, " ");
            let command_and_args: Vec<String> = command_and_args_
                                                .map(|part| part.to_string())
                                                .collect();

            if let (Some(command_name), Some(command_args)) = (command_and_args.get(0), command_and_args.get(1)) {

                let command_name_safe = command_name
                    .replace(".","")
                    .replace("/","")
                    .replace("~","");

                if let Ok(ast) = RHAI_INSTANCE.rhai_.compile_file(format!("{}/{}.rhai", &SETTING.rhai_exts_path, command_name_safe).into()) {

                    let mut scope = Scope::new();


                    if let Ok(res) = RHAI_INSTANCE.rhai_.call_fn::<String>(&mut scope, &ast, "handle", (command_args.clone(), )) {
                        lines_output.push(res);
                    } else {
                        lines_output.push(line.to_string());
                    }
                } else {
                    lines_output.push(line.to_string());
                }
            } else {
                lines_output.push(line.to_string());
            }
        } else {
            lines_output.push(line.to_string());
        }
    }

    lines_output.join("\n").to_string()
}

