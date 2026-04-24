#!/usr/bin/env python
# splits an existing nqrustbackup postgres dump into single files per table

import re

outfile = open("header", "w")

copy = re.compile("^COPY (\w+)")

for line in open('nqrustbackup.sql'):
    m = re.match(copy, line)
    if m:
      print line, m.group(1)
      outfile.close()
      outfile = open(m.group(1) + ".sql", "w")
      outfile.write(line)
    else: 
      outfile.write(line)
