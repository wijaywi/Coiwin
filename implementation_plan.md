# Goal
Implementasi CLI (Command Line Interface) Wallet yang mendukung **Transaksi Hybrid (ECDSA + Dilithium)**. 
Fitur ini bertujuan sebagai *Proof of Concept* (PoC) di mana sebuah transaksi harus ditandatangani menggunakan algoritma klasik (ECDSA/secp256k1) dan post-quantum (Dilithium) secara bersamaan untuk mencapai keamanan yang maksimal (Hybrid Security).

## User Review Required
> [!IMPORTANT]
> Mohon tinjau rencana ini. Pengembangan CLI akan menambah library baru seperti `clap` (untuk *command line*), `k256` (untuk ECDSA), dan `serde` (untuk serialisasi format JSON).

## Open Questions
> [!WARNING]
> 1. **Penyimpanan Kunci (Key Storage)**: Untuk PoC ini, apakah Anda ingin *keypair* (ECDSA & Dilithium) disimpan dalam file lokal (misal: `wallet.json`) atau cukup dicetak (print) di terminal saja setiap kali di-generate? Saya merekomendasikan menyimpannya di file lokal agar mudah digunakan saat membuat transaksi.
> 2. **Format Transaksi**: Apakah transaksi akan direpresentasikan dalam format `JSON` agar mudah dibaca, atau format binary mentah? (Rekomendasi: `JSON` untuk kemudahan PoC).

## Proposed Changes

### 1. Dependensi (`Cargo.toml`)
#### [MODIFY] [Cargo.toml](file:///D:/zzzzzzzzzzz%20AntiGravity/Coiwin--main/Cargo.toml)
- Menambahkan dependensi baru:
  - `clap = { version = "4", features = ["derive"] }` (untuk *parsing* argumen CLI)
  - `k256 = { version = "0.13", features = ["ecdsa"] }` atau `secp256k1` (untuk algoritma ECDSA)
  - `serde`, `serde_json` (untuk menyimpan *wallet* dan membuat format *payload* transaksi)
  - `rand` (untuk *random number generator* ECDSA)

### 2. Modul Wallet & Hybrid Transaction
#### [NEW] [src/wallet/hybrid_tx.rs](file:///D:/zzzzzzzzzzz%20AntiGravity/Coiwin--main/src/wallet/hybrid_tx.rs)
- Membuat `struct HybridKeypair` yang menyimpan kombinasi *private key* dan *public key* dari ECDSA maupun Dilithium.
- Membuat `struct HybridTransaction` yang berisi data *sender*, *receiver*, *amount*, serta 2 jenis *signature* (ECDSA signature dan Dilithium signature).
- Implementasi fungsi `sign_transaction` yang menggunakan kedua *private keys* untuk menghasilkan 2 *signatures*.
- Implementasi fungsi `verify_transaction` yang memvalidasi kedua *signatures* menggunakan masing-masing *public key*.

#### [NEW] [src/wallet/mod.rs](file:///D:/zzzzzzzzzzz%20AntiGravity/Coiwin--main/src/wallet/mod.rs)
- Mengekspor `hybrid_tx`.

#### [MODIFY] [src/lib.rs](file:///D:/zzzzzzzzzzz%20AntiGravity/Coiwin--main/src/lib.rs)
- Menambahkan `pub mod wallet;`.

### 3. CLI Executable (`src/bin/wallet.rs`)
#### [NEW] [src/bin/wallet.rs](file:///D:/zzzzzzzzzzz%20AntiGravity/Coiwin--main/src/bin/wallet.rs)
Membuat file *binary* terpisah agar proyek dapat dijalankan sebagai *library* maupun aplikasi CLI. CLI akan memiliki 3 perintah utama:
- `wallet generate` -> Menghasilkan kunci hibrida baru dan menyimpannya ke `wallet.json`.
- `wallet transfer <to> <amount>` -> Membuat *Hybrid Transaction*, menandatanganinya, dan menyimpannya ke file `tx.json`.
- `wallet verify <tx_file>` -> Membaca transaksi dari file dan memverifikasi integritas kedua tanda tangan (Hybrid Verification).

## Verification Plan

### Automated Tests
- Menulis unit tests di dalam `src/wallet/hybrid_tx.rs` (atau `tests/wallet_tests.rs`) untuk memastikan transaksi di-*sign* dan di-*verify* dengan benar menggunakan kedua algoritma.
- Memastikan mutasi (*tampering*) pada data transaksi akan menggagalkan tahap verifikasi.

### Manual Verification
- Menjalankan CLI dan mensimulasikan *flow* secara penuh: 
  `cargo run --bin wallet -- generate`
  `cargo run --bin wallet -- transfer 0xPenerima 100`
  `cargo run --bin wallet -- verify tx.json`
