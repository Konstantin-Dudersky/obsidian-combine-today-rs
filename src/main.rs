use obsidian_combine_today_rs::*;

fn main() {
    let args = Args::new().unwrap();

    let note_reader = NoteReader::new();
    let get_timestamp = GetTimestamp::new();
    let file_writer = FileWriter::new(&args.target_file);

    let runner =
        Runner::new(note_reader, get_timestamp, file_writer, &args.folders);

    runner.run().unwrap();
}
