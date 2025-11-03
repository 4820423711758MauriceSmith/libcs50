import ctypes
import os
import sys

# 加载cs50库
libcs50 = None
try:
    # 尝试从系统库路径加载
    libcs50 = ctypes.CDLL('libcs50.so')
except OSError:
    # 尝试从当前目录加载
    try:
        if sys.platform.startswith('win'):
            libcs50 = ctypes.CDLL('cs50.dll')
        elif sys.platform == 'darwin':
            libcs50 = ctypes.CDLL('libcs50.dylib')
        else:
            libcs50 = ctypes.CDLL(os.path.join(os.path.dirname(__file__), 'libcs50.so'))
    except OSError as e:
        raise ImportError(f"无法加载cs50库: {e}")

# 定义函数类型
libcs50.get_char.argtypes = [ctypes.c_char_p]
libcs50.get_char.restype = ctypes.c_char

libcs50.get_double.argtypes = [ctypes.c_char_p]
libcs50.get_double.restype = ctypes.c_double

libcs50.get_float.argtypes = [ctypes.c_char_p]
libcs50.get_float.restype = ctypes.c_float

libcs50.get_int.argtypes = [ctypes.c_char_p]
libcs50.get_int.restype = ctypes.c_int

libcs50.get_long.argtypes = [ctypes.c_char_p]
libcs50.get_long.restype = ctypes.c_long

libcs50.get_long_long.argtypes = [ctypes.c_char_p]
libcs50.get_long_long.restype = ctypes.c_longlong

libcs50.get_string.argtypes = [ctypes.c_char_p]
libcs50.get_string.restype = ctypes.c_char_p

libcs50.free_string.argtypes = [ctypes.c_char_p]
libcs50.free_string.restype = None

def get_char(prompt=""):
    """Prompt user for a single character."""
    c_prompt = prompt.encode('utf-8') if prompt else None
    result = libcs50.get_char(c_prompt)
    if result == 0xFF:  # CHAR_MAX
        raise EOFError("输入错误或EOF")
    return chr(result)

def get_double(prompt=""):
    """Prompt user for a double."""
    c_prompt = prompt.encode('utf-8') if prompt else None
    result = libcs50.get_double(c_prompt)
    if result == ctypes.c_double.max:  # DBL_MAX
        raise EOFError("输入错误或EOF")
    return result

def get_float(prompt=""):
    """Prompt user for a float."""
    c_prompt = prompt.encode('utf-8') if prompt else None
    result = libcs50.get_float(c_prompt)
    if result == ctypes.c_float.max:  # FLT_MAX
        raise EOFError("输入错误或EOF")
    return result

def get_int(prompt=""):
    """Prompt user for an integer."""
    c_prompt = prompt.encode('utf-8') if prompt else None
    result = libcs50.get_int(c_prompt)
    if result == 0x7FFFFFFF:  # INT_MAX
        raise EOFError("输入错误或EOF")
    return result

def get_long(prompt=""):
    """Prompt user for a long integer."""
    c_prompt = prompt.encode('utf-8') if prompt else None
    result = libcs50.get_long(c_prompt)
    if result == 0x7FFFFFFFFFFFFFFF:  # LONG_MAX
        raise EOFError("输入错误或EOF")
    return result

def get_long_long(prompt=""):
    """Prompt user for a long long integer."""
    c_prompt = prompt.encode('utf-8') if prompt else None
    result = libcs50.get_long_long(c_prompt)
    if result == 0x7FFFFFFFFFFFFFFF:  # LLONG_MAX
        raise EOFError("输入错误或EOF")
    return result

def get_string(prompt=""):
    """Prompt user for a string."""
    c_prompt = prompt.encode('utf-8') if prompt else None
    c_string = libcs50.get_string(c_prompt)
    if c_string is None:
        raise EOFError("输入错误或EOF")
    
    # 将C字符串转换为Python字符串并释放内存
    python_string = c_string.decode('utf-8')
    libcs50.free_string(c_string)
    return python_string