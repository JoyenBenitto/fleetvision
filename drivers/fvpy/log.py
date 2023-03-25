import logging
import serial
import string
import time

serial_port = serial.Serial('/dev/ttyUSB2', 9600)

while True:
    serial_data = serial_port.readline()
    print(serial_data)
