use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;
use std::sync::mpsc;
use std::thread;

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_hanlde = thread::spawn(move || read::read_loop(&infile, stats_tx));
    let stats_hanlde = thread::spawn(move || stats::stats_loop(silent, stats_rx, write_tx));
    let write_hanlde = thread::spawn(move || write::write_loop(&outfile, write_rx));

    // crash if any threads have crashed
    // `.join()` returns a `thread::Result<io::Result<()>>`
    let read_io_result = read_hanlde.join().unwrap();
    let stats_io_result = stats_hanlde.join().unwrap();
    let write_io_result = write_hanlde.join().unwrap();

    // Return an error if any threads returned an error
    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
