# encoding:utf8
from genericpath import isfile
import os
import platform
import sys


def get_params(plat):
    return [
        "--release",
        "--target={}".format("i686-pc-windows-msvc" if sys.platform ==
                             "win32" else "i686-pc-windows-msvc")
    ]


def get_cargo():
    ret = os.system("cargo")
    print(ret)


if __name__ == "__main__":
    print(get_cargo())

# os.popen("cargo ")
# os.system('''cargo build  --release --target=i686-pc-windows-msvc''')
