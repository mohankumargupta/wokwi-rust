import sys
import shutil
from pathlib import Path

def main():
    build_base = Path("target") / "xtensa-esp32-none-elf" / "debug" 
    firmware_bin = "epaper"
    firmware_bin_full_path = build_base / firmware_bin 

    dest_dir = Path("./firmware")
    dest_dir.mkdir(exist_ok=True)
    print(firmware_bin_full_path)
    shutil.copy2(firmware_bin_full_path, dest_dir / firmware_bin)

if __name__ == "__main__":
    main()
