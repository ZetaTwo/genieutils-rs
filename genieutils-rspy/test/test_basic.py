#!/usr/bin/env python3

import genieutils_rspy

with open('test/empires2_x2_p1.dat', 'rb') as fin:
    compressed = fin.read()

decompressed = genieutils_rspy.DatFile.decompress(compressed)
datfile = genieutils_rspy.DatFile.parse(decompressed)


print(dir(datfile))
print(datfile.version)
print(datfile.float_ptr_terrain_tables)
datfile.float_ptr_terrain_tables.append(1)
print(datfile.float_ptr_terrain_tables)
print(datfile.sounds)
print(type(datfile.sounds[0]))
print(dir(datfile.sounds[0]))


for player_colour in datfile.player_colours:
    print(player_colour)

#serialized = genieutils_rspy.DatFile.serialize(datfile)
#print(f'Round-trip works: {decompressed == serialized}')

"""

print(f'Loom gold cost: {datfile.datfile.techs[22].resource_costs[0].amount}')
datfile.datfile.techs[22].resource_costs[0].amount = 69
print(f'Loom gold cost: {datfile.datfile.techs[22].resource_costs[0].amount}')

print(f'Version: {datfile.datfile.version}')
serialized = datfile.serialize()

print(f'Round-trip works: {decompressed == serialized}')
"""
