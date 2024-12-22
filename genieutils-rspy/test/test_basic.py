#!/usr/bin/env python3

from genieutils_rspy import DatFile
import time

with open('test/empires2_x2_p1.dat', 'rb') as fin:
    compressed = fin.read()

for _ in range(3):
    t0 = time.time()
    decompressed = DatFile.decompress(compressed)
    datfile = DatFile.parse(decompressed)
    print(time.time()-t0)
    serialized = datfile.serialize()
    t1 = time.time()
    print(f'Round-trip works: {decompressed == serialized}, took: {t1-t0:.2f} seconds')
