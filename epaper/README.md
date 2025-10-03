# Epaper Demo

### Custom font

1. Convert ttf to bdf 
```
otf2bdf -v -p 10 -r 72  -o p10.bdf p10.ttf
```

```txt
bdfconv [options] filename
-h          Display this help
-v          Print log messages
-b <n>      Font build mode, 0: proportional, 1: common height, 2: monospace, 3: multiple of 8, 4: 5x7 mode
-f <n>      Font format, 0: ucglib font, 1: u8g2 font, 2: u8g2 uncompressed 8x8 font (enforces -b 3)
-m 'map'    Unicode ASCII mapping
-M 'mapfile'    Read Unicode ASCII mapping from file 'mapname'
-u 'utf8file'    Include all characters from utf8 text file
-o <file>   C output font file
-k <file>   C output file with kerning information
-p <%>      Minimum distance for kerning in percent of the global char width (lower values: Smaller gaps, more data)
-x <n>      X-Offset for 8x8 font sub-glyph extraction (requires -f 2, default 0)
-y <n>      Y-Offset for 8x8 font sub-glyph extraction (requires -f 2, default 0)
-th <n>     Horizontal size of the 8x8 glyphs (requires -f 2, default 1)
-tv <n>     Vertical size of the 8x8 glyphs (requires -f 2, default 1)
-n <name>   C indentifier (font name)
-d <file>   Overview picture: Enable generation of bdf.tga and assign BDF font <file> for description
-l <margin> Overview picture: Set left margin
-g <glyphs> Overview picture: Set glyphs per line (default: 16)
-a          Overview picture: Additional font information (background, orange&blue dot)
-t          Overview picture: Test string (Woven silk pyjamas exchanged for blue quartz.)
-r          Runtime test
```



2. To view bdf file
```
fontforge p10.pdf
```
3. convert bdf to u8g font 
```
./u8g2/tools/font/bdfconv/bdfconv -v -b 0 -f 1 -m "32,65-90" -k p10.k -n "p10" -o p10.c p10.bdf
```

4. use ai to convert C octal array to rust array.
