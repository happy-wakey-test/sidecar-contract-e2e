use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn isolated_installed_binary_reaches_ready_without_stdout() {
    let binary = std::env::var("HAPPY_WAKEY_INSTALLED_BIN")
        .expect("scripts/certify.sh supplies the isolated installed binary");
    let product = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    product.set_nonblocking(true).unwrap();
    let product_addr = product.local_addr().unwrap();
    let stop = Arc::new(AtomicBool::new(false));
    let server_stop = Arc::clone(&stop);
    let server = thread::spawn(move || {
        while !server_stop.load(Ordering::Relaxed) {
            match product.accept() {
                Ok((mut stream, _)) => {
                    let mut request = [0_u8; 1024];
                    let _ = stream.read(&mut request);
                    let _ = stream.write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok",
                    );
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(_) => return,
            }
        }
    });

    let reservation = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let sidecar_addr = reservation.local_addr().unwrap();
    drop(reservation);
    let mut child = Command::new(binary)
        .env_clear()
        .env("HAPPY_WAKEY_SIDECAR_BIND", sidecar_addr.to_string())
        .env("HAPPY_WAKEY_SIDECAR_PRODUCT_KIND", "api")
        .env(
            "HAPPY_WAKEY_SIDECAR_PRODUCT_PROBE_URL",
            format!("http://{product_addr}/healthz"),
        )
        .env(
            "HAPPY_WAKEY_SHARED_AUTH_BASE_URL",
            "https://auth.example.test",
        )
        .env(
            "HAPPY_WAKEY_OPTO_SYNC_BASE_URL",
            "http://opto-sync.sync.svc",
        )
        .env("HAPPY_WAKEY_SIDECAR_INTERVAL_MS", "250")
        .env("HAPPY_WAKEY_SIDECAR_SUCCESS_THRESHOLD", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let deadline = Instant::now() + Duration::from_secs(8);
    let mut ready = false;
    while Instant::now() < deadline {
        if let Ok(mut stream) =
            TcpStream::connect_timeout(&sidecar_addr, Duration::from_millis(200))
        {
            stream
                .set_read_timeout(Some(Duration::from_millis(500)))
                .unwrap();
            stream
                .write_all(b"GET /readyz HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
                .unwrap();
            let mut response = String::new();
            let _ = stream.read_to_string(&mut response);
            if response.starts_with("HTTP/1.1 200") {
                ready = true;
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    let _ = child.kill();
    let output = child.wait_with_output().unwrap();
    stop.store(true, Ordering::Relaxed);
    server.join().unwrap();
    assert!(ready, "installed sidecar never exposed ready capability");
    assert!(output.stdout.is_empty(), "sidecar wrote reserved stdout");
}
