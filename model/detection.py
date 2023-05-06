#!/usr/bin/python3

from ultralytics import YOLO

model = YOLO('best.pt')  # load a custom model
while True:
    image = input()
    results = model(image, verbose = False)
    for prediction in results[0].boxes.cls:
        print(results[0].names[prediction.item()])
    print('.')
