const sharp = require('sharp');
const fs = require('fs');
const path = require('path');

const iconsDir = path.join(__dirname, 'src-tauri', 'icons');

// Create a simple gradient icon using Sharp
async function createIcon() {
    // Create SVG
    const svg = `
    <svg width="512" height="512" xmlns="http://www.w3.org/2000/svg">
        <defs>
            <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
            </linearGradient>
        </defs>
        <rect width="512" height="512" rx="100" fill="url(#grad)"/>
        <text x="256" y="300" font-family="Arial, sans-serif" font-size="180" font-weight="bold" text-anchor="middle" fill="white">EC</text>
    </svg>`;

    // Create directory if not exists
    if (!fs.existsSync(iconsDir)) {
        fs.mkdirSync(iconsDir, { recursive: true });
    }

    // Generate different sizes
    const sizes = [
        { name: '32x32.png', size: 32 },
        { name: '128x128.png', size: 128 },
        { name: '128x128@2x.png', size: 256 },
        { name: 'icon.png', size: 512 }
    ];

    for (const { name, size } of sizes) {
        await sharp(Buffer.from(svg))
            .resize(size, size)
            .png()
            .toFile(path.join(iconsDir, name));
        console.log(`Created ${name}`);
    }

    // Create ICO for Windows (using 32x32)
    await sharp(Buffer.from(svg))
        .resize(32, 32)
        .png()
        .toFile(path.join(iconsDir, 'icon.ico'));
    console.log('Created icon.ico');

    // Create ICNS placeholder for macOS
    fs.writeFileSync(path.join(iconsDir, 'icon.icns'), Buffer.from([0x69, 0x63, 0x6E, 0x73]));
    console.log('Created icon.icns (placeholder)');
}

createIcon().catch(console.error);