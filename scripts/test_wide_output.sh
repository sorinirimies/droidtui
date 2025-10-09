#!/bin/bash

# Test script to generate wide output for testing DroidTUI text wrapping functionality

echo "=== DroidTUI Wide Content Test ==="
echo "This script generates wide lines to test text wrapping and layout handling."
echo ""

# Generate some wide lines that would overflow normal terminal width
echo "Very long line that exceeds normal terminal width and should be wrapped properly: Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."

echo "Another extremely long line with lots of technical information that might be typical of process output: PID=12345 PPID=1 USER=system COMMAND=/system/bin/some_very_long_process_name_that_goes_on_and_on VSZ=123456789 RSS=987654321 STATE=R"

echo ""
echo "=== Simulated Process Output (Wide Format) ==="
printf "%-8s %-8s %-12s %-6s %-8s %-8s %-60s\n" "PID" "PPID" "USER" "STATE" "VSZ" "RSS" "COMMAND"
printf "%-8s %-8s %-12s %-6s %-8s %-8s %-60s\n" "--------" "--------" "------------" "------" "--------" "--------" "------------------------------------------------------------"
printf "%-8s %-8s %-12s %-6s %-8s %-8s %-60s\n" "1234" "1" "root" "S" "12345" "6789" "/system/bin/init --early-mount --second-stage"
printf "%-8s %-8s %-12s %-6s %-8s %-8s %-60s\n" "5678" "1234" "system" "R" "98765" "4321" "/system/bin/some_very_long_system_process_name_here"
printf "%-8s %-8s %-12s %-6s %-8s %-8s %-60s\n" "9012" "1" "android" "S" "11111" "2222" "com.android.systemui.with.a.very.long.package.name.here"

echo ""
echo "=== Long Path Examples ==="
echo "/system/framework/very/long/path/to/some/android/system/component/that/has/many/subdirectories/file.apk"
echo "/data/data/com.example.app.with.very.long.package.name/files/cache/temporary/nested/deep/structure/file.txt"
echo "/storage/emulated/0/Android/data/com.another.app.with.extremely.long.name/files/Downloads/SomeVeryLongFileName.pdf"

echo ""
echo "=== Wide Table Format ==="
echo "PACKAGE_NAME                                    VERSION   SIZE      INSTALL_DATE         FLAGS"
echo "com.android.chrome                             91.0.4472 156MB     2023-10-15 14:30:22  system,user"
echo "com.google.android.gms.with.very.long.name     21.39.13  89MB      2023-10-14 09:15:43  system,persistent"
echo "com.example.application.with.extremely.long.package.name.here 1.2.3 45MB 2023-10-13 16:45:12 user,debuggable"

echo ""
echo "=== Network Information (Wide) ==="
echo "INTERFACE     IP_ADDRESS        NETMASK           GATEWAY          DNS_PRIMARY      DNS_SECONDARY    MTU   STATE"
echo "wlan0         192.168.1.100     255.255.255.0     192.168.1.1      8.8.8.8          8.8.4.4          1500  UP"
echo "rmnet_data0   10.123.456.789    255.255.255.255   10.123.456.1     208.67.222.222   208.67.220.220   1420  UP"

echo ""
echo "=== End of Wide Content Test ==="
echo "Total lines with various widths to test wrapping and scrolling functionality."
