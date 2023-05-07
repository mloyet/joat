#!/usr/bin/python3

import sys
def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)

from ultralytics import YOLO

model = YOLO('/home/pi/joat/model/best.pt')  # load a custom model
while True:
    image = input()
    results = model(image, verbose = False)
    for prediction in results[0].boxes.cls:
        print(results[0].names[prediction.item()])
    print('.')
