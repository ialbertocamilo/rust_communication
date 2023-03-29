use std::collections::HashMap;
use std::fs::File;
use env_file_reader::read_file;

pub struct Config {
    env : HashMap<String, String>
}

impl Config {

    pub fn new() -> Config {
        Self{env:read_file("./.env").unwrap()}
    }

    pub fn get (&self, var:&str) -> String {
        self.env[var]
    }
}