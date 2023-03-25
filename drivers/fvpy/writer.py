import subprocess

file_ = open('log.txt', 'w+')
subprocess.run('python3 log.py', shell=True, stdout=file_)
file_.close()
