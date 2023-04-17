import sys
import binascii

data = sys.stdin.read().replace('\r\n', ' ').replace('0x','').split(', ')[:-1]
raw = [binascii.unhexlify(word) for word in data]

for word in raw:
    sys.stdout.buffer.write(word)
