extern crate hex;
extern crate sha3;
extern crate hmacsha;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;

use futures::future;
use hmacsha::HmacSha;
use sha3::Sha3_256;
use sha3::digest::Output;

fn path_reference(path_cycle: String) -> fs::File{
    fs::File::open(path_cycle).expect("file not found")
}

const KEY: &'static str = "Arch repository";

fn compiler_arch(comp: String){
    let mut hashc = path_reference(comp);
    let mut reader_file_rep = String::new();
    hashc.read_to_string(&mut reader_file_rep).unwrap();
    let mut fatalf_hash_sha = HmacSha::from(KEY, &mut reader_file_rep, Sha3_256::default());
    let result_hashc = fatalf_hash_sha.compute_digest();
    let hash_load = hex::encode(result_hashc);
    println!("{}", hash_load);

    if fs::create_dir("arch").is_err(){
        let mut new_filec = File::create("foo.arch").unwrap();
        new_filec.write(hash_load.as_bytes()).unwrap();
    }else{
        let mut new_filec = File::create("foo.arch").unwrap();
        new_filec.write(hash_load.as_bytes()).unwrap();
    }
}

use std::process::Command;

fn update_file() -> std::io::Result<()>{
    if cfg!(target_os = "linux"){
        Command::new("mv").args(["foo.arch", "arch"]).output().expect("error");
    }else{
        Command::new("move").args(["foo.arch", "arch"]).output().expect("error");    
    }

    Ok(())
} 

extern crate  rand;

use rand::prelude::*;

fn execute_rand() -> std::io::Result<()>{
    let mut gen_file = rand::random::<char>();
    fs::rename("arch/foo.arch", gen_file.to_string())?;

    Ok(())
}

extern crate futures;

fn fn_async(){

}

async fn pause_thread() -> dyn Fn() {

   fn_async()
}
fn main() {
    let query_selector = env::args().collect::<Vec<String>>();

    match query_selector[1].as_str(){
        "--query" => {
            match query_selector[2].as_str(){
                "-u" => {
                   let _path_u = String::new();
                   match query_selector[3].as_str(){
                       _path_u => {
                            compiler_arch(_path_u.to_string());
                            println!("return -> {}", _path_u);
                            update_file().unwrap();
                            let function_slow = &execute_rand();
                            pause_thread(function_slow);
                            
                       },
                       _ => ()
                   }
            },
            _ => ()

            }
        },
        _ => ()
    }

}
