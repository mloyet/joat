import sys
import binascii

data = sys.stdin.read().replace('\r\n', ' ').replace('\n', ' ').replace('0x','').replace(' ', '').split(',')
raw = [binascii.unhexlify(word) for word in data]

for word in raw:
    sys.stdout.buffer.write(word)
