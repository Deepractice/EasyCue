const fs = require('fs');
const path = require('path');

// Simple SVG icon
const svg = `<svg width="512" height="512" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
    </linearGradient>
  </defs>
  <rect width="512" height="512" rx="100" fill="url(#grad)"/>
  <text x="256" y="320" font-family="Arial, sans-serif" font-size="200" font-weight="bold" text-anchor="middle" fill="white">EC</text>
</svg>`;

// Create icons directory
const iconsDir = path.join(__dirname, 'src-tauri', 'icons');
if (!fs.existsSync(iconsDir)) {
    fs.mkdirSync(iconsDir, { recursive: true });
}

// Save SVG
fs.writeFileSync(path.join(iconsDir, 'icon.svg'), svg);

// Create simple PNG placeholder (would need a proper converter in production)
fs.writeFileSync(path.join(iconsDir, 'icon.png'), Buffer.from([
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A
]));

// Create required icon files (placeholders)
const sizes = ['32x32', '128x128', '128x128@2x'];
sizes.forEach(size => {
    fs.writeFileSync(path.join(iconsDir, `${size}.png`), Buffer.from([
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A
    ]));
});

// Create .ico and .icns placeholders
fs.writeFileSync(path.join(iconsDir, 'icon.ico'), Buffer.from([0x00, 0x00]));
fs.writeFileSync(path.join(iconsDir, 'icon.icns'), Buffer.from([0x69, 0x63, 0x6E, 0x73]));

console.log('Icon placeholders created!');
console.log('Note: For production, use proper icon generation tools.');