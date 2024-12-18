#!/usr/bin/env python3

import zlib

with open('empires2_x2_p1.dat', 'rb') as fin:
    data = fin.read()

with open('empires2_x2_p1.unpack', 'wb') as fout:
    fout.write(zlib.decompress(data, wbits=-15))
