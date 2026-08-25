use std::fs;
use std::process::Command;

#[test]
fn includes_firmware_revision_in_csv_output() {
    let unique_id = format!("{}", std::process::id());
    let input_path = std::env::temp_dir().join(format!("disk_parser_input_{unique_id}.txt"));
    let output_path = std::env::temp_dir().join(format!("disk_parser_output_{unique_id}.csv"));
    let input = "-- Physical Disk Information - Disk: #12: SAMSUNG MZILG7T6HBLA/A07 --\n\
        Hard Disk Number . . . . . . . . . . . . . . . . : 12\n\
        Hard Disk Model ID . . . . . . . . . . . . . . . : SAMSUNG MZILG7T6HBLA/A07\n\
        Firmware Revision  . . . . . . . . . . . . . . . : GXG3\n\
        Hard Disk Serial Number  . . . . . . . . . . . . : S70KNA0YB01828\n";

    fs::write(&input_path, input).expect("write test input");

    let result = Command::new(env!("CARGO_BIN_EXE_disk_parser"))
        .arg(&input_path)
        .arg(&output_path)
        .output()
        .expect("run disk_parser");

    let csv = fs::read_to_string(&output_path).expect("read generated CSV");
    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    assert!(result.status.success(), "disk_parser failed: {result:?}");
    assert_eq!(
        csv,
        "Disk #,Model,Firmware Revision,Serial Number,Power On Time,Lifetime Writes,Manufacture Date\n\
         12,SAMSUNG MZILG7T6HBLA/A07,GXG3,S70KNA0YB01828,Unknown,Unknown,Unknown\n"
    );
}
