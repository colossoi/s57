#!/usr/bin/env nu
# Process S-57 chart ZIP files and report geographic extents
#
# Requirements: Set S57_CLI environment variable to path of s57-cli binary
#
# Usage: nu process_chart_zip.nu <zip_url>
# Example: S57_CLI=../target/release/s57-cli nu process_chart_zip.nu https://charts.noaa.gov/ENCs/IN_ENCs.zip

def main [
    zip_url: string  # URL to download S-57 chart ZIP file
] {
    print $"Processing charts from: ($zip_url)"

    # Create temporary directory
    let temp_dir = (mktemp -d)
    print $"Using temporary directory: ($temp_dir)"

    # Extract filename from URL
    let zip_filename = ($zip_url | path basename)
    let zip_path = ($temp_dir | path join $zip_filename)

    # Download the ZIP file
    print $"Downloading ($zip_filename)..."
    http get $zip_url | save -f $zip_path

    # Extract the ZIP file
    print $"Extracting charts..."
    unzip -q $zip_path -d $temp_dir

    # Find all S-57 base cell files (*.000)
    print "\nFinding S-57 chart files..."
    let chart_files = (glob $"($temp_dir)/**/*.000")
    print $"Found ($chart_files | length) chart files\n"

    # Get s57-cli path from environment variable
    let cli_path = if ("S57_CLI" in $env) {
        $env.S57_CLI
    } else {
        print "Error: S57_CLI environment variable not set"
        print "Set it to the path of the s57-cli binary, e.g.:"
        print "  export S57_CLI=/path/to/s57-cli"
        return
    }

    # Process each chart file and collect results
    print "Chart Extents:"
    print "============================================"
    print $"(char nl){'File':<40} {'Min Lat':>10} {'Max Lat':>10} {'Min Lon':>11} {'Max Lon':>11}"
    print $"{'-' * 40} {'-' * 10} {'-' * 10} {'-' * 11} {'-' * 11}"

    for file in $chart_files {
        let chart_name = ($file | path basename)

        # Run s57-cli extent command (file path must come before subcommand)
        let result = (do -i {
            ^$cli_path $file extent
            | complete
        })

        if $result.exit_code != 0 {
            print $"($chart_name | fill -a l -c ' ' -w 40) [Error: ($result.stderr | str trim)]"
            continue
        }

        # Parse the extent output
        # Looking for lines like: "  Latitude:  41.1234567 to 41.5678901"
        let lat_line = ($result.stdout | lines | where { |line| $line =~ '^\s*Latitude:' } | first)
        let lon_line = ($result.stdout | lines | where { |line| $line =~ '^\s*Longitude:' } | first)

        if ($lat_line | is-empty) or ($lon_line | is-empty) {
            print $"($chart_name | fill -a l -c ' ' -w 40) [No extent data]"
            continue
        }

        # Extract numbers using regex
        let lat_parts = ($lat_line | parse -r 'Latitude:\s+(?<min>-?\d+\.\d+)\s+to\s+(?<max>-?\d+\.\d+)')
        let lon_parts = ($lon_line | parse -r 'Longitude:\s+(?<min>-?\d+\.\d+)\s+to\s+(?<max>-?\d+\.\d+)')

        if ($lat_parts | length) == 0 or ($lon_parts | length) == 0 {
            print $"($chart_name | fill -a l -c ' ' -w 40) [Parse error]"
            continue
        }

        let min_lat = ($lat_parts | first | get min)
        let max_lat = ($lat_parts | first | get max)
        let min_lon = ($lon_parts | first | get min)
        let max_lon = ($lon_parts | first | get max)

        print $"($chart_name | fill -a l -c ' ' -w 40) ($min_lat | fill -a r -c ' ' -w 10) ($max_lat | fill -a r -c ' ' -w 10) ($min_lon | fill -a r -c ' ' -w 11) ($max_lon | fill -a r -c ' ' -w 11)"
    }

    # Cleanup
    print $"\n(char nl)Cleaning up temporary directory..."
    rm -rf $temp_dir
    print "Done!"
}
