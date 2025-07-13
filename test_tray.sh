#!/bin/bash

echo "Starting System Tray Application Test"
echo "====================================="
echo ""
echo "This will run the system tray app for 10 seconds"
echo "Look for a blue circle icon with 'A' in your system tray!"
echo ""
echo "System tray locations:"
echo "- KDE: Bottom-right corner"
echo "- GNOME: Top-right corner (needs AppIndicator extension)"
echo "- XFCE: Usually bottom-right"
echo ""
echo "Starting in 3 seconds..."
sleep 3

# Run the app in background
timeout 10 cargo run --release &

# Wait and show status
sleep 2
echo ""
echo "The app should now be running!"
echo "Check your system tray area for the icon."
echo "Right-click on it to see the menu."
echo ""
echo "The app will automatically stop in 8 seconds..."

# Wait for timeout
wait

echo ""
echo "Test completed!"