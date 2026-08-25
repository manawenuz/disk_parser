# disk_parser

`disk_parser` converts a physical-disk information report into a CSV file.

## Usage

```text
disk_parser <input_file> <output_csv>
```

For example:

```text
disk_parser physical-disks.txt disks.csv
```

The input report can include a firmware line such as:

```text
Firmware Revision  . . . . . . . . . . . . . . . : GXG3
```

The generated CSV contains these columns:

| Column | Description |
| --- | --- |
| `Disk #` | Physical disk number |
| `Model` | Hard disk model identifier |
| `Firmware Revision` | Firmware revision reported by the disk |
| `Serial Number` | Hard disk serial number |
| `Power On Time` | Estimated time the disk has been powered on |
| `Lifetime Writes` | Reported lifetime writes |
| `Manufacture Date` | Reported manufacture year/week |

Fields that are not present in the input report are written as `Unknown`.

## Windows AMD64 releases

Push a version tag such as `v0.1.0` to build and publish a Windows AMD64 release:

```text
git tag v0.1.0
git push origin v0.1.0
```

The GitHub Actions workflow builds for `x86_64-pc-windows-msvc`, packages
`disk_parser.exe` as `disk_parser-windows-amd64.zip`, and attaches the ZIP to
the GitHub Release. Branch pushes also produce the same ZIP as a workflow
artifact.
