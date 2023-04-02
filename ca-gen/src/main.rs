use std::fs;
use std::io::{Write};
use base64::{engine::general_purpose, Engine as _};

fn main() {
	let chia_ca_crt = fs::read("~/.chia/mainnet/config/ssl/ca/chia_ca.crt").unwrap();
	let chia_ca_key = fs::read("~/.chia/mainnet/config/ssl/ca/chia_ca.key").unwrap();
	let private_ca_crt = fs::read("~/.chia/mainnet/config/ssl/ca/private_ca.crt").unwrap();
	let private_ca_key = fs::read("~/.chia/mainnet/config/ssl/ca/private_ca.key").unwrap();
	let chia_ca_crt = general_purpose::STANDARD.encode(&chia_ca_crt);
	let chia_ca_key = general_purpose::STANDARD.encode(&chia_ca_key);
	let private_ca_crt = general_purpose::STANDARD.encode(&private_ca_crt);
	let private_ca_key = general_purpose::STANDARD.encode(&private_ca_key);

	let mut output = fs::File::create("../chia-secret.yaml").unwrap();
	writeln!(output, "---");
	writeln!(output, "apiVersion: v1");
	writeln!(output, "kind: Secret");
	writeln!(output, "metadata:");
	writeln!(output, "  namespace: chia");
	writeln!(output, "  name: chia-secret");
	writeln!(output, "type: Opaque");
	writeln!(output, "data:");
	writeln!(output, "  chia_ca.crt: {}", chia_ca_crt);
	writeln!(output, "  chia_ca.key: {}", chia_ca_key);
	writeln!(output, "  private_ca.crt: {}", private_ca_crt);
	writeln!(output, "  private_ca.key: {}", private_ca_key);
}
