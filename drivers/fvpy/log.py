import logging
import serial
import string
import time

serial_port = serial.Serial('/dev/ttyACM0', 9600)

while True:
    serial_data = serial_port.readline()
    serial_data = serial_data.split(',')
    serial_data[0] = serial_data[0][2:]
    serial_data[2] = serial_data[2][:-2] 
    print(serial_data)
