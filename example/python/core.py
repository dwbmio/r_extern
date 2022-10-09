#encoding:utf8 

from ctypes import cdll

lib = cdll.LoadLibrary("E:\\private_work\\r_extern\\target\\debug\\r_extern.dll")
print(lib.add(2))