#!/usr/bin/env python3

import genieutils_rspy

with open('test/empires2_x2_p1.dat', 'rb') as fin:
    compressed = fin.read()

decompressed = genieutils_rspy.DatFile.decompress(compressed)
datfile = genieutils_rspy.DatFile.parse(decompressed)

print(dir(datfile))

print(f'Loom gold cost: {datfile.datfile.techs[22].resource_costs[0].amount}')
datfile.datfile.techs[22].resource_costs[0].amount = 69
print(f'Loom gold cost: {datfile.datfile.techs[22].resource_costs[0].amount}')

print(f'Version: {datfile.datfile.version}')
serialized = datfile.serialize()

print(f'Round-trip works: {decompressed == serialized}')
