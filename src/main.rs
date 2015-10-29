#![feature(convert)]

extern crate hyper;
extern crate time;

use hyper::{Server, Post};
use hyper::net::Fresh;
use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use std::env;
use std::fs::File;
use std::process::Command;

macro_rules! header {
    ($res:expr, $h:expr, $v:expr) =>
    ($res.headers_mut().set_raw($h, vec![$v.to_vec()]));
}

macro_rules! end {
    ($res:expr, $stat:expr) => ({
        *$res.status_mut() = $stat;
        $res.send(b"").unwrap();
        return ()
    });
}

fn now() -> String {
    format!("{}", time::now().rfc3339())
}

fn hook(req: Request, mut res: Response<Fresh>) {
    header!(res, "Access-Control-Allow-Methods", b"POST");
    header!(res, "Access-Control-Allow-Origin", b"*");
    header!(res, "X-Content-Type-Options", b"nosniff");
    header!(res, "X-Download-Options", b"noopen");
    header!(res, "X-Frame-Options", b"DENY");
    header!(res, "X-XSS-Protection", b"1; mode=block");

    match req.method {
        Post => {},
        _ => end!(res, StatusCode::BadRequest)
    }

    let mut file = format!("hooks{}", req.uri);
    if file == "hooks/".to_string() {
        file = "hooks/default".to_string();
    }

    let handle = match File::open(&file) {
        Ok(h) => h,
        Err(e) => {
            println!("[{}] Could not open file {}: {}", now(), file, e);
            end!(res, StatusCode::NotFound);
        }
    };

    let metadata = match handle.metadata() {
        Ok(d) => d,
        Err(e) => {
            println!("[{}] Could get file metadata for {}: {}", now(), file, e);
            end!(res, StatusCode::NotFound);
        }
    };

    if !metadata.is_file() {
        println!("[{}] Not a file: {}", now(), file);
        end!(res, StatusCode::NotFound);
    }

    println!("[{}] Running {}...", now(), &file);
    let status = match Command::new(&file).status() {
        Ok(c) => c,
        Err(e) => {
            println!("[{}] Error executing {}: {}", now(), file, e);
            end!(res, StatusCode::NoContent);
        }
    };

    println!("[{}] Exit status from {}: {}", now(), file, status);
    end!(res, StatusCode::NoContent);
}

fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => "5000".to_string()
    };
    
    match Server::http(format!("0.0.0.0:{}", &port).as_str()) {
        Ok(s) => {
            println!("[{}] Server started on port {}", now(), port);
            let _ = s.handle(hook);
        },
        Err(e) => { println!("[{}] Could not start server: {}", now(), e); }
    }
}
