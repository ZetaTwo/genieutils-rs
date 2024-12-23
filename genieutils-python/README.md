# Genieutils-RS Python Bindings

## Performance

```
$ /usr/bin/time -v python test/test_basic.py
Round-trip works: True, parsing: 1.06s, serializing: 2.05s, total: 3.11s
Round-trip works: True, parsing: 1.10s, serializing: 1.46s, total: 2.56s
Round-trip works: True, parsing: 0.49s, serializing: 1.45s, total: 1.94s
...
Maximum resident set size (kbytes): 724788

$ /usr/bin/time -v python test/test_compare.py 
Round-trip works: True, parsing: 13.13s, serializing: 4.89s, total: 18.02s
Round-trip works: True, parsing: 15.68s, serializing: 4.67s, total: 20.35s
Round-trip works: True, parsing: 14.08s, serializing: 4.74s, total: 18.82s
...
Maximum resident set size (kbytes): 1413852
```
