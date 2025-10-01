set shell := ["sh", "-c"]
set windows-shell := ["powershell", "-c"]

_main:
    @just --list

hello:
    cd hello-led; cargo build

hell0_troubleshooting:
    cd hello-led; cargo tree --edges features -p hello-led

led:
    cd led-effects; cargo build


