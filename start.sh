#!/bin/bash

echo "🚀 Starting Ruziniu Tools with Tailwind CSS..."

# Kill any existing processes on port 1420
echo "🔧 Cleaning up any existing processes..."
pkill -f "vite" 2>/dev/null || true
pkill -f "ruziniu-tools" 2>/dev/null || true

# Wait a moment for processes to fully stop
sleep 2

echo "✨ Launching application..."
npm run tauri dev

echo "👋 Application stopped."