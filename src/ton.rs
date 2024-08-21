use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use regex::Regex;

use crate::{constants, utils};

// model validator status data
#[derive(Debug)]
pub struct ValidatorStatus {
    pub network: String,
    pub validator_index: i32,
    pub validator_address: String,
    pub validator_balance: f64,
    pub validator_outofsync: i32,
}

// model pool data
#[derive(Debug)]
pub struct PoolStatus {
    pub pool_address: String,
    pub pool_active: bool,
    pub pool_balance: f64,
}

// wrapper around the mytonctrl binary
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

    // parse data from `mytonctrl status`
    pub fn status(self: &mut Self) -> ValidatorStatus {
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
            Regex::new(r"^Local validator out of sync: (.*) s").unwrap();

        // initialize attributes
        let mut network: String = "".into();
        let mut validator_index: i32 = -1;
        let mut validator_address: String = "".into();
        let mut validator_balance: f64 = 0.0;
        let mut validator_outofsync: i32 = 0;

        // parse metrics and break
        for line in reader.lines() {
            let contents = &line.expect("Failed to read line!");

            // get current network (mainnet|testnet)
            if let Some(captures) = pattern_network.captures(contents) {
                if captures.len() > 1 {
                    network = utils::decolorize(&captures[1]);
                }
            }
            // get validator address
            else if let Some(captures) = pattern_validator_address.captures(contents) {
                if captures.len() > 1 {
                    validator_address = utils::decolorize(&captures[1]);
                }
            }
            // get validator index
            else if let Some(captures) = pattern_validator_index.captures(contents) {
                if captures.len() > 1 {
                    validator_index = utils::decolorize(&captures[1])
                        .parse()
                        .expect("Unable to parse validator index!");
                }
            }
            // get validator balance
            else if let Some(captures) = pattern_validator_balance.captures(contents) {
                if captures.len() > 1 {
                    validator_balance = utils::decolorize(&captures[1])
                        .parse()
                        .expect("Unable to parse validator balance!");
                }
            }
            // get validator out of sync duration in seconds
            // NOTE: it is crucial for this to be the last match to break out of reading the stdout buffer
            //       as it does not have an EOF (due to mytonctrl being a shell-like interface)
            else if let Some(captures) = pattern_validator_outofsync.captures(contents) {
                if captures.len() > 1 {
                    validator_outofsync = utils::decolorize(&captures[1])
                        .parse()
                        .expect("Unable to parse validator out of sync duration!");
                }
                break;
            }
        }

        ValidatorStatus {
            network,
            validator_index,
            validator_address,
            validator_balance,
            validator_outofsync,
        }
    }
}
