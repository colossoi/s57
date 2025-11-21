#!/usr/bin/env nu
# Process S-57 chart ZIP files and index features to SQLite database
#
# Requirements: Set S57_CLI environment variable to path of s57-cli binary
#
# Usage: nu process_chart_zip.nu <zip_url> [database_path]
# Example: S57_CLI=../target/release/s57-cli nu process_chart_zip.nu https://charts.noaa.gov/ENCs/IN_ENCs.zip charts.db

def main [
    zip_url: string              # URL to download S-57 chart ZIP file
    database_path?: string       # Optional SQLite database path (default: charts.db)
] {
    let db_path = if ($database_path | is-empty) { "charts.db" } else { $database_path }
    print $"Processing charts from: ($zip_url)"
    print $"Database: ($db_path)"

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

    # Process each chart file and index to database
    print "\nIndexing Features to Database:"
    print "============================================\n"

    mut charts_processed = 0
    mut charts_failed = 0
    mut total_features = 0

    for file in $chart_files {
        let chart_name = ($file | path basename)
        print $"Processing ($chart_name)..."

        # Run s57-cli extent command with database parameter
        let result = (do -i {
            ^$cli_path $file extent --database $db_path
            | complete
        })

        if $result.exit_code != 0 {
            print $"  ERROR: Failed to process chart"
            $charts_failed = $charts_failed + 1
        } else {
            # Parse output: "INDEXED: 1173/1173"
            let output = ($result.stdout | str trim)
            print $"  ($output)"

            let parsed = ($output | parse -r 'INDEXED: (?<indexed>\d+)/(?<total>\d+)')
            if ($parsed | length) > 0 {
                let indexed = ($parsed | first | get indexed | into int)
                $total_features = $total_features + $indexed
            }
            $charts_processed = $charts_processed + 1
        }
        print ""
    }

    # Print summary
    print "============================================"
    print "Summary:"
    print $"  Charts processed: ($charts_processed)"
    print $"  Charts failed: ($charts_failed)"
    print $"  Total features indexed: ($total_features)"
    print $"  Database: ($db_path)"

    # Cleanup
    print $"\n(char nl)Cleaning up temporary directory..."
    rm -rf $temp_dir
    print "Done!"
}
