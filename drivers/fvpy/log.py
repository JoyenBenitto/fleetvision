import logging
import serial
import string
import time
import subprocess

serial_port = serial.Serial('/dev/ttyACM0', 9600)

while True:
    serial_data = str(serial_port.readline())
    serial_data = serial_data.split(',')
    serial_data[0] = serial_data[0][2:]
    serial_data[2] = serial_data[2][:-2] 
    print(serial_data)
