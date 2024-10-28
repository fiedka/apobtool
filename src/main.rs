use clap::Parser;
use std::{fs, io};

mod apob;
use apob::{ApobHeader, ApobSystemMemoryMapType};

/// APOB debug tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Print
    #[arg(required = false, short, long)]
    print: bool,

    /// Print verbosely
    #[arg(required = false, short, long)]
    verbose: bool,

    /// File to read
    #[arg(index = 1)]
    file: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = args.file;
    let data = fs::read(file).unwrap();
    // let verbose = args.verbose;
    println!("ROM size: {}", data.len());

    let magic = &data[0..4];
    if magic != apob::APOB_SIGNATURE.as_bytes() {
        panic!("signature not found, got {:02x?}", magic);
    }

    let header: &ApobHeader = plain::from_bytes(&data).expect("failed to cast ApobHeader");
    println!("{:#X?}", header);

    let mem: &ApobSystemMemoryMapType = plain::from_bytes(&data[header.sys_map_offset as usize..])
        .expect("failed to cast ApobSystemMemoryMapType");
    println!("{:#X?}", mem);

    Ok(())
}
