from pathlib import Path
for i in range(32, 127):
    if chr(i) == '/':
        continue
    f = "{:02X}{}{:02X}".format(i, chr(i), i)
    Path(f).touch()
