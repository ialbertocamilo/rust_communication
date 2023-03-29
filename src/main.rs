#![feature(slice_take)]

mod config;

use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::mem::transmute;
use std::net::UdpSocket;
use std::process::exit;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant, SystemTime};

use aes::{Aes128, Aes256};
use aes::cipher::{BlockDecrypt, BlockEncrypt, Key, KeyInit};
use aes::cipher::crypto_common::rand_core::OsRng;
use aes::cipher::generic_array::GenericArray;
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use interprocess::os::unix::fifo_file::create_fifo;
use interprocess::os::unix::signal::SignalHandler;
use libc::{rand, regmatch_t, SIG_ERR, sighandler_t, SIGINT, SIGKILL, signal, SIGTERM, time, time_t};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use regex::{Match, Regex};
use std::env;
use crate::config::Config;

unsafe fn udp_server() {
    let udp = UdpSocket::bind("127.0.0.1:8989").unwrap();
    let mut buf = [0; 255];
    loop {
        let (len, addr) = udp.recv_from(&mut buf).unwrap();
        let time = Instant::now();
        println!("{:?} bytes received", len);
        println!("{:?} ", String::from_utf8(buf.to_vec()).unwrap().replace("\0", ""));
        println!("{:?}", time.elapsed().as_micros());
        udp.send_to("ping".as_bytes(), addr).unwrap();
    }
}

fn unix_socket() {
    let local_socket = LocalSocketListener::bind("/tmp/localsocket.v5").unwrap();
    {
        // local_socket.set_nonblocking(true).unwrap();
        loop {
            let mut buffer = String::new();
            let mut accept = local_socket.accept().unwrap();
            accept.read_to_string(&mut buffer).unwrap();

            println!("Servidor iniciado");
            println!("{:?}", buffer);
            accept.write_all(b"Respuesta de servidor").expect("Error al responder al cliente");
        }
    }
}

fn detect_close(signal: u32) {
    println!("Funcion al finalizar la SIGNAL SIGINT");
    println!("Signal nro {:?}", signal);
    exit(1);
}


fn  random_alpha()->String{
    (0..16).map(|_|  rand::thread_rng().sample(Alphanumeric) as char).collect()
}
fn cipher(msg : &str) -> Result<(), Box<dyn Error>> {
    let mut rnd = rand::thread_rng();
    // let str_key :[u8;16]= rnd.gen(); //Random key generate bytes
    // let chars =random_alpha();//Random key generate



    let mut static_key= Config::new().get("KEY_CODE").as_bytes() as [u8;16] ;
    let key = GenericArray::from(static_key);
    println!("Bytes key {:?}", static_key);
    let mut block = GenericArray::from([50u8; 16]);

    let cipher = Aes128::new(&key);
    cipher.encrypt_block(&mut block);
    println!("Encrypt block {:?}", block);
    cipher.decrypt_block(&mut block);
    println!("Decrypt block blocks {:?}", block);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // let mut buf = String::new();
    // // let rex= String::from(r"\s(.*?)\s");
    // let rex = String::from(r"\S+(?=\s*=)");
    //
    // let mut f = File::open("/proc/stat").unwrap();
    // f.read_to_string(&mut buf).unwrap();
    // let _= buf.split("").map(|item| {
    //     println!("{:?}", item.to_string());
    // });
    //
    // unsafe { udp_server(); }


    // spawn(unix_socket);
    // unsafe {
    //     let sig = signal(SIGINT, detect_close as sighandler_t);
    //     if sig == SIG_ERR {
    //         println!("Error  en signal");
    //     }
    //     loop {
    //         println!("Sleeping...");
    //         sleep(Duration::from_millis(600))
    //     }
    // };

    cipher("gaa");

    Ok(())
}
