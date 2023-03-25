import logging
import serial
import string
import time
import sys
import os

serial_port = serial.Serial('/dev/ttyACM0', 9600)
o_stdout = sys.stdout
if os.path.exists("log.csv"):
    os.remove("log.csv")

while True:
    serial_data = serial_port.readline()
    serial_data = serial_data.decode()
    serial_data = serial_data.rstrip() 
    print(serial_data)
    file = open('log.csv', 'a+')
    file.write(serial_data)
    file.write('\n')
    file.close()
    time.sleep(0.1)

serial_data.close()
