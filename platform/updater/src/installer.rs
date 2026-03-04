//! Package downloader and installer for self-update.

use super::checker::UpdateInfo;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Download `info.download_url`, verify SHA-256, and launch the installer.
///
/// On success this function does **not** return — the current process exits
/// after launching the installer so the user sees a single window.
pub fn download_and_install(info: &UpdateInfo) -> Result<(), Box<dyn std::error::Error>> {
    let tmp = std::env::temp_dir().join(format!("fpe_update_{}", &info.new_version));
    std::fs::create_dir_all(&tmp)?;

    let filename = info
        .download_url
        .split('/')
        .last()
        .unwrap_or("update_package");
    let dest: PathBuf = tmp.join(filename);

    info!("Downloading update {} to {}", info.new_version, dest.display());
    download_file(&info.download_url, &dest)?;

    verify_sha256(&dest, &info.sha256)?;
    info!("SHA-256 verified for update package");

    launch_installer(&dest)?;
    std::process::exit(0);
}

fn download_file(url: &str, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let resp = ureq::get(url).call()?;
    let mut reader = resp.into_reader();
    let mut file = std::fs::File::create(dest)?;
    std::io::copy(&mut reader, &mut file)?;
    Ok(())
}

fn verify_sha256(path: &Path, expected: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Read;

    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let digest = hasher.finalize_hex();
    if digest.eq_ignore_ascii_case(expected) {
        Ok(())
    } else {
        Err(format!("SHA-256 mismatch: expected {expected}, got {digest}").into())
    }
}

/// Tiny inline SHA-256 using Rust's standard library (no external crate needed).
struct Sha256 {
    state: [u32; 8],
    buf: Vec<u8>,
}

impl Sha256 {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    fn new() -> Self {
        Self {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
                0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
            ],
            buf: Vec::new(),
        }
    }

    fn update(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    fn finalize_hex(mut self) -> String {
        let bit_len = (self.buf.len() as u64) * 8;
        self.buf.push(0x80);
        while self.buf.len() % 64 != 56 {
            self.buf.push(0x00);
        }
        self.buf.extend_from_slice(&bit_len.to_be_bytes());

        for chunk in self.buf.chunks(64) {
            let mut w = [0u32; 64];
            for (i, b) in chunk.chunks(4).enumerate().take(16) {
                w[i] = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
            }
            for i in 16..64 {
                let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
                let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
                w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
            }
            let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;
            for i in 0..64 {
                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = (e & f) ^ ((!e) & g);
                let t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(Self::K[i]).wrapping_add(w[i]);
                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let t2 = s0.wrapping_add(maj);
                h = g; g = f; f = e;
                e = d.wrapping_add(t1);
                d = c; c = b; b = a;
                a = t1.wrapping_add(t2);
            }
            let ns = [a, b, c, d, e, f, g, h];
            for (i, v) in ns.iter().enumerate() {
                self.state[i] = self.state[i].wrapping_add(*v);
            }
        }

        self.state
            .iter()
            .flat_map(|v| v.to_be_bytes())
            .fold(String::with_capacity(64), |mut s, b| {
                s.push_str(&format!("{:02x}", b));
                s
            })
    }
}

fn launch_installer(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("msiexec")
            .arg("/i")
            .arg(path)
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).spawn()?;
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        warn!("Self-update installer launch not supported on this platform");
        return Err("unsupported platform".into());
    }
    Ok(())
}
