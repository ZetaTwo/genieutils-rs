#!/usr/bin/env python3

from genieutils.datfile import DatFile
from genieutils.common import ByteHandler
import time
import zlib

with open('test/empires2_x2_p1.dat', 'rb') as fin:
    compressed = fin.read()

for _ in range(3):
    t0 = time.time()
    decompressed = zlib.decompress(compressed, wbits=-15)
    byte_handler = ByteHandler(memoryview(decompressed))
    datfile = DatFile.from_bytes(byte_handler)
    t1 = time.time()
    serialized = datfile.to_bytes()
    t2 = time.time()
    print(f'Round-trip works: {decompressed == serialized}, parsing: {t1-t0:.2f}s, serializing: {t2-t1:.2f}s, total: {t2-t0:.2f}s')
