use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use regex::Regex;

use crate::{constants, utils};

// model validator status data
#[derive(Debug, Default)]
pub struct ValidatorStatus {
    pub network: String,
    pub index: i32,
    pub address: String,
    pub balance: f64,
    pub outofsync: i32,
}

// model pool data
#[derive(Debug, Default)]
pub struct PoolStatus {
    pub name: String,
    pub address: String,
    pub active: bool,
    pub balance: f64,
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
    pub fn validator_status(self: &mut Self) -> ValidatorStatus {
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
        let mut validator_data = ValidatorStatus::default();

        // parse metrics and break
        for line in reader.lines() {
            let contents = &line.expect("Failed to read line!");

            // get current network (mainnet|testnet)
            if let Some(captures) = pattern_network.captures(contents) {
                if captures.len() > 1 {
                    validator_data.network = utils::decolorize(&captures[1]);
                }
            }
            // get validator address
            else if let Some(captures) = pattern_validator_address.captures(contents) {
                if captures.len() > 1 {
                    validator_data.address = utils::decolorize(&captures[1]);
                }
            }
            // get validator index
            else if let Some(captures) = pattern_validator_index.captures(contents) {
                if captures.len() > 1 {
                    validator_data.index = utils::decolorize(&captures[1])
                        .parse()
                        .expect("Unable to parse validator index!");
                }
            }
            // get validator balance
            else if let Some(captures) = pattern_validator_balance.captures(contents) {
                if captures.len() > 1 {
                    validator_data.balance = utils::decolorize(&captures[1])
                        .parse()
                        .expect("Unable to parse validator balance!");
                }
            }
            // get validator out of sync duration in seconds
            // NOTE: it is crucial for this to be the last match to break out of reading the stdout buffer
            //       as it does not have an EOF (due to mytonctrl being a shell-like interface)
            else if let Some(captures) = pattern_validator_outofsync.captures(contents) {
                if captures.len() > 1 {
                    validator_data.outofsync = utils::decolorize(&captures[1])
                        .parse()
                        .expect("Unable to parse validator out of sync duration!");
                }
                break;
            }
        }

        validator_data
    }

    pub fn pool_status(self: &mut Self) -> Vec<PoolStatus> {
        // fetch stdin of the mytonctrl process
        let stdin = self.process.stdin.as_mut().expect("Failed to open stdin!");

        // fetch data from the process
        stdin
            .write_all(b"pools_list\n")
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
        let pattern_header =
            Regex::new(r"^Name\s+Status\s+Balance\s+Version\s+Address\s+$").unwrap();

        // initialize attributes
        let mut pool_data: Vec<PoolStatus> = vec![];

        // flip to true when pool header pattern is encountered
        // signifies the pool data region is being read in the buffer
        let mut pool_encountered = false;

        // parse metrics and break
        for line in reader.lines() {
            let contents = &utils::decolorize(&line.expect("Failed to read line!"));

            // if buffer is in the pool data region, parse and store it
            if pool_encountered {
                let pool: Vec<&str> = contents.split(" ").filter(|a| !a.is_empty()).collect();
                if !pool.is_empty() {
                    pool_data.push(PoolStatus {
                        name: pool[0].to_string(),
                        active: pool[1] == "active",
                        balance: pool[2].parse().unwrap(),
                        address: pool[4].to_string(),
                    })
                }
            }

            // start reading pool data once header is encountered
            if pattern_header.is_match(contents) {
                pool_encountered = true;
            }

            // stop reading from buffer when the final empty line is reached
            if pool_encountered && contents.is_empty() {
                break;
            }
        }

        pool_data
    }
}
