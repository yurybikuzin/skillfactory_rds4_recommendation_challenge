#!/usr/bin/env python3
chunk_size = 2048
per_row = 16
row_count = int(chunk_size / per_row)
for i in range(0,row_count):
  for j in range(0,per_row):
      print(i*per_row + j, end=', ')
  print()
print()
