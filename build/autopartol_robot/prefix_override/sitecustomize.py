import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/yonghui/chapt7/chapt7_ws/src/install/autopartol_robot'
