# Epaper Demo

### Custom font

1. Convert ttf to bdf 
```
otf2bdf -v -p 10 -r 72  -o p10.bdf p10.ttf
```
2. To view bdf file
```
fontforge p10.pdf
```
3. convert bdf to u8g font 
```
bdfconv -v -b 0 -f 1 -m "65-90" -n "p10" -o p10.c p10.bdf
```
