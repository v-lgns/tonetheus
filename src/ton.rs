use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use regex::Regex;

use crate::{constants, utils};

pub struct MyTonCtrl {
    pub process: Child,
}

impl MyTonCtrl {
    pub fn new() -> Self {
        let process = Command::new("mytonctrl")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start process!");

        // wait for initialization
        thread::sleep(Duration::from_secs(constants::MYTONCTRL_INIT_DELAY));

        // TODO: check if process was initialized properly, else fail

        Self { process }
    }

    pub fn status(self: &mut Self) {
        // fetch stdin of the mytonctrl process
        let stdin = self.process.stdin.as_mut().expect("Failed to open stdin!");

        // fetch data from the process
        stdin
            .write_all(b"status\n")
            .expect("Failed to write to stdin!");

        // wait for command output
        thread::sleep(Duration::from_secs(constants::MYTONCTRL_CMD_DELAY));

        // read output from the process
        let stdout = self
            .process
            .stdout
            .as_mut()
            .ok_or("Failed to read from stdout!")
            .unwrap();
        let reader = BufReader::new(stdout);

        // regexes
        let pattern_network = Regex::new(r"^Network name: (.*)").unwrap();
        let pattern_validator_address =
            Regex::new(r"^Local validator wallet address: (.*)").unwrap();
        let pattern_validator_index = Regex::new(r"^Validator index: (.*)").unwrap();
        let pattern_validator_balance =
            Regex::new(r"^Local validator wallet balance: (.*)").unwrap();
        let pattern_validator_outofsync =
            Regex::new(r"^Local validator out of sync: (.*)").unwrap();

        // parse metrics and break
        for line in reader.lines() {
            let contents = &line.expect("Failed to read line!");

            // get current network (mainnet|testnet)
            if let Some(captures) = pattern_network.captures(contents) {
                if captures.len() > 1 {
                    let network = utils::decolorize(&captures[1]);
                    dbg!(network);
                }
            }
            // get validator address
            else if let Some(captures) = pattern_validator_address.captures(contents) {
                if captures.len() > 1 {
                    let validator_address = utils::decolorize(&captures[1]);
                    dbg!(validator_address);
                }
            }
            // get validator index
            else if let Some(captures) = pattern_validator_index.captures(contents) {
                if captures.len() > 1 {
                    let validator_index = utils::decolorize(&captures[1]);
                    dbg!(validator_index);
                }
            }
            // get validator balance
            else if let Some(captures) = pattern_validator_balance.captures(contents) {
                if captures.len() > 1 {
                    let validator_balance = utils::decolorize(&captures[1]);
                    dbg!(validator_balance);
                }
            }
            // get validator out of sync duration in seconds
            // NOTE: it is crucial for this to be the last match to break out of reading the stdout buffer
            //       as it does not have an EOF (due to mytonctrl being a shell-like interface)
            else if let Some(captures) = pattern_validator_outofsync.captures(contents) {
                if captures.len() > 1 {
                    let validator_outofsync = utils::decolorize(&captures[1]);
                    dbg!(validator_outofsync);
                }
                break;
            }
        }
    }
}
