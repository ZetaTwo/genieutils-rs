#!/usr/bin/env python3

from genieutils_rs import DatFile
import time

with open('test/empires2_x2_p1.dat', 'rb') as fin:
    compressed = fin.read()

for _ in range(3):
    t0 = time.time()
    decompressed = DatFile.decompress(compressed)
    datfile = DatFile.parse(decompressed)
    t1 = time.time()
    serialized = datfile.serialize()
    t2 = time.time()
    print(f'Round-trip works: {decompressed == serialized}, parsing: {t1-t0:.2f}s, serializing: {t2-t1:.2f}s, total: {t2-t0:.2f}s')
