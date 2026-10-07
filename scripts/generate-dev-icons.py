#!/usr/bin/env python3
import os
import struct
import zlib

def make_rgba_png(filename, width, height, r=10, g=132, b=255, a=255):
    header = b"\x89PNG\r\n\x1a\n"
    
    def chunk(chunk_type: bytes, data: bytes) -> bytes:
        return (
            struct.pack(">I", len(data))
            + chunk_type
            + data
            + struct.pack(">I", zlib.crc32(chunk_type + data) & 0xffffffff)
        )
    
    # 8-bit depth, color type 6 (RGBA: 4 bytes per pixel)
    ihdr_data = struct.pack(">II", width, height) + b"\x08\x06\x00\x00\x00"
    ihdr = chunk(b"IHDR", ihdr_data)
    
    raw_scanline = b"\x00" + bytes([r, g, b, a]) * width
    raw_data = raw_scanline * height
    idat = chunk(b"IDAT", zlib.compress(raw_data))
    iend = chunk(b"IEND", b"")
    
    os.makedirs(os.path.dirname(filename), exist_ok=True)
    with open(filename, "wb") as f:
        f.write(header + ihdr + idat + iend)

def make_dummy_ico(filename):
    # Minimal 1-image ICO with embedded PNG
    os.makedirs(os.path.dirname(filename), exist_ok=True)
    png_path = filename.replace(".ico", "-tmp.png")
    make_rgba_png(png_path, 32, 32)
    with open(png_path, "rb") as f:
        png_bytes = f.read()
    if os.path.exists(png_path):
        os.remove(png_path)
        
    ico_header = struct.pack("<HHH", 0, 1, 1) # Reserved, Type 1 (ICO), 1 image
    ico_dir = struct.pack(
        "<BBBBHHII",
        32, 32, 0, 0, 1, 32, len(png_bytes), 22 # 6 (header) + 16 (dir entry) = 22 offset
    )
    with open(filename, "wb") as f:
        f.write(ico_header + ico_dir + png_bytes)

def make_dummy_icns(filename):
    # Minimal ICNS container with ic07 (128x128 PNG)
    os.makedirs(os.path.dirname(filename), exist_ok=True)
    png_path = filename.replace(".icns", "-tmp.png")
    make_rgba_png(png_path, 128, 128)
    with open(png_path, "rb") as f:
        png_bytes = f.read()
    if os.path.exists(png_path):
        os.remove(png_path)
    
    chunk_type = b"ic07"
    chunk_len = len(png_bytes) + 8
    total_len = chunk_len + 8
    header = b"icns" + struct.pack(">I", total_len)
    body = chunk_type + struct.pack(">I", chunk_len) + png_bytes
    with open(filename, "wb") as f:
        f.write(header + body)

def main():
    icons_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "src-tauri", "icons"))
    os.makedirs(icons_dir, exist_ok=True)
    
    make_rgba_png(os.path.join(icons_dir, "32x32.png"), 32, 32)
    make_rgba_png(os.path.join(icons_dir, "128x128.png"), 128, 128)
    make_rgba_png(os.path.join(icons_dir, "128x128@2x.png"), 256, 256)
    make_rgba_png(os.path.join(icons_dir, "icon.png"), 512, 512)
    make_dummy_ico(os.path.join(icons_dir, "icon.ico"))
    make_dummy_icns(os.path.join(icons_dir, "icon.icns"))
    print(f"✅ Generated valid 32-bit RGBA developer icons in {icons_dir}")

if __name__ == "__main__":
    main()
