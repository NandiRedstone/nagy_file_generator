use rayon::prelude::*;
use rand::{Rng, thread_rng};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Mutex;

fn main() -> std::io::Result<()> {
    let mut input_string = String::new();
    println!("Írd be, mekkora fájlt szeretnél gigabájtban:");
    std::io::stdin().read_line(&mut input_string).expect("Nem sikerült beolvasni a sort");
    let file_size_gb: u64 = input_string.trim().parse().expect("Kérlek, írj be egy számot!");

    let path = "nagy_file.txt";
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    let writer = Mutex::new(writer);

    let num_bytes_per_number = 11;
    let target_size_bytes = file_size_gb * 1024 * 1024 * 1024; // GB-t byte-ra konvertálva
    let total_numbers = target_size_bytes / num_bytes_per_number as u64;

    (0..total_numbers).into_par_iter().for_each(|_| {
        let mut rng = thread_rng();
        let num: u64 = rng.gen_range(0..=9999999999);
        let mut writer_guard = writer.lock().unwrap();
        writeln!(writer_guard, "{}", num).expect("Nem sikerült írni a számot");
    });

    Ok(())
}
