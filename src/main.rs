use anyhow::{anyhow, Error};
use pico_args::Arguments;
use std::process::{Command, Stdio};

mod config;

use config::Args;
use std::io::Write;

fn main() -> Result<(), Error> {
    let mut parsed = Arguments::from_env();

    let args = Args {
        command: parsed.subcommand()?,
        private_key: parsed.opt_value_from_str(["-s", "--seed"])?,
        server: parsed.opt_value_from_str(["-S", "--server"])?,
        subject: parsed.opt_value_from_str(["-t", "--subject"])?,
        message: parsed.opt_value_from_str(["-m", "--message"])?,
        exec: parsed.opt_value_from_str(["-x", "--exec"])?,
    };

    let maybe_exec = args.exec.clone();

    if let Some(command) = args.command {
        match command.as_str() {
            "publish" => {
                let nc = nats::connect(&args.server.unwrap())?;
                nc.publish(&args.subject.unwrap(), &args.message.unwrap())?;
            }
            "subscribe" => {
                let nc = nats::connect(&args.server.unwrap())?;
                let sub = nc.subscribe(&args.subject.unwrap())?;
                while let Some(msg) = sub.next() {
                    let maybe_string = String::from_utf8(msg.data);
                    if maybe_string.is_err() {
                        return Err(anyhow!("data is not utf-8"));
                    }
                    let s = maybe_string.unwrap();
                    match maybe_exec.clone() {
                        None => println!("debug: {}", s),
                        Some(program) => {
                            let mut cmd = Command::new(program);
                            cmd.stdin(Stdio::piped());
                            let mut child = cmd.spawn()?;
                            {
                                let stdin = child.stdin.as_mut().unwrap();
                                // TODO(cm): buffer the writes, make this robust
                                stdin.write_all(&s.as_bytes())?;
                            }
                            let status = child.wait()?;
                            println!("child process exit code: {:?}", status.code());
                        }
                    }
                }
            }
            _ => {
                msg_exit(&format!("unrecognized command: {}", command));
            }
        }
    }

    Ok(())
}

fn msg_exit(msg: &str) {
    println!("{}", msg);
    std::process::exit(1);
}
