#!/usr/bin/env python3

from genieutils_rs import DatFile
import sys

with open('test/empires2_x2_p1.dat', 'rb') as fin:
    compressed = fin.read()

try:
    decompressed = DatFile.decompress(compressed)
except Exception as e:
    print(e.with_traceback())
    sys.exit(1)

datfile = DatFile.parse(decompressed)


print(dir(datfile))
print(datfile.version)
print(datfile.float_ptr_terrain_tables)
#datfile.float_ptr_terrain_tables.append(1)
print(datfile.float_ptr_terrain_tables)
print(datfile.sounds)
print(type(datfile.sounds[0]))
print(dir(datfile.sounds[0]))

for player_colour in datfile.player_colours:
    print(player_colour)

serialized1 = datfile.serialize()
print(f'Round-trip works: {decompressed == serialized1}')

print(f'Loom gold cost: {datfile.techs[22].resource_costs[0].amount}')
datfile.techs[22].resource_costs[0].amount = 69
print(f'Loom gold cost: {datfile.techs[22].resource_costs[0].amount}')

print(f'Version: {datfile.version}')
serialized2 = datfile.serialize()
serialized3 = datfile.serialize()

print(f'Round-trip works: {decompressed == serialized2}')
print(f'Round-trip works: {serialized2 == serialized3}')
