#!/usr/bin/env python3

import genieutils_rspy

with open('test/empires2_x2_p1.dat', 'rb') as fin:
    compressed = fin.read()

x = genieutils_rspy.DatFile.parse_compressed(compressed)
print(f'Version: {x.version}')
print(dir(x))

recompressed = x.pack()

print(type(compressed))
print(type(recompressed))
print(compressed == recompressed)
