use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Debug)]
struct DiskInfo {
    disk_num: String,
    model: String,
    firmware_revision: String,
    serial: String,
    power_on_time: String,
    lifetime_writes: String,
    manufacture_date: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <output_csv>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut disks = Vec::new();
    let mut current_disk = DiskInfo {
        firmware_revision: "Unknown".to_string(),
        power_on_time: "Unknown".to_string(),
        lifetime_writes: "Unknown".to_string(),
        manufacture_date: "Unknown".to_string(),
        ..Default::default()
    };

    let re_disk_num = Regex::new(r"Hard Disk Number [\. ]+ : (\d+)")?;
    let re_model = Regex::new(r"Hard Disk Model ID [\. ]+ : (.+)")?;
    let re_firmware_revision = Regex::new(r"Firmware Revision [\. ]+ : (.+)")?;
    let re_serial = Regex::new(r"Hard Disk Serial Number [\. ]+ : (.+)")?;
    // Days/hours/minutes are each optional; some disks report only days and
    // hours, and some lines end with "(estimated)".
    let re_power_on = Regex::new(r"Power On Time [\. ]+ : (.+)")?;
    let re_power_on_days = Regex::new(r"(\d+) days")?;
    let re_power_on_hours = Regex::new(r"(\d+) hours")?;
    let re_power_on_minutes = Regex::new(r"(\d+) minutes")?;
    let re_writes = Regex::new(r"Lifetime Writes [\. ]+ : (.+)")?;
    let re_mfg_date = Regex::new(r"Manufacture date \(year/week\)\s+(\d{4}/\d{2})")?;

    for line in reader.lines() {
        let line = line?;
        if line.contains("-- Physical Disk Information - Disk:") {
            if !current_disk.disk_num.is_empty() {
                disks.push(current_disk);
                current_disk = DiskInfo {
                    firmware_revision: "Unknown".to_string(),
                    power_on_time: "Unknown".to_string(),
                    lifetime_writes: "Unknown".to_string(),
                    manufacture_date: "Unknown".to_string(),
                    ..Default::default()
                };
            }
        }

        if let Some(caps) = re_disk_num.captures(&line) {
            current_disk.disk_num = caps[1].trim().to_string();
        } else if let Some(caps) = re_model.captures(&line) {
            current_disk.model = caps[1].trim().to_string();
        } else if let Some(caps) = re_firmware_revision.captures(&line) {
            current_disk.firmware_revision = caps[1].trim().to_string();
        } else if let Some(caps) = re_serial.captures(&line) {
            current_disk.serial = caps[1].trim().to_string();
        } else if let Some(caps) = re_power_on.captures(&line) {
            let value = caps[1].trim();
            let mut parts: Vec<String> = Vec::new();
            if let Some(d) = re_power_on_days.captures(value) {
                parts.push(format!("{} days", &d[1]));
            }
            if let Some(h) = re_power_on_hours.captures(value) {
                parts.push(format!("{}h", &h[1]));
            }
            if let Some(m) = re_power_on_minutes.captures(value) {
                parts.push(format!("{}m", &m[1]));
            }
            if !parts.is_empty() {
                let mut power_on_time = parts.join(", ");
                if value.contains("(estimated)") {
                    power_on_time.push_str(" (estimated)");
                }
                current_disk.power_on_time = power_on_time;
            }
        } else if let Some(caps) = re_writes.captures(&line) {
            current_disk.lifetime_writes = caps[1].trim().to_string();
        } else if let Some(caps) = re_mfg_date.captures(&line) {
            current_disk.manufacture_date = caps[1].trim().to_string();
        }
    }

    if !current_disk.disk_num.is_empty() {
        disks.push(current_disk);
    }

    let mut wtr = csv::Writer::from_path(output_path)?;
    wtr.write_record(&[
        "Disk #",
        "Model",
        "Firmware Revision",
        "Serial Number",
        "Power On Time",
        "Lifetime Writes",
        "Manufacture Date",
    ])?;

    for d in &disks {
        wtr.write_record(&[
            &d.disk_num,
            &d.model,
            &d.firmware_revision,
            &d.serial,
            &d.power_on_time,
            &d.lifetime_writes,
            &d.manufacture_date,
        ])?;
    }
    wtr.flush()?;

    println!(
        "Successfully parsed {} disks into {}",
        disks.len(),
        output_path
    );
    Ok(())
}
