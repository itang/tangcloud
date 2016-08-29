# .*. encoding: utf-8 .*.

from datetime import datetime
from fabric.api import *


def test():
    """test"""
    local('http post ":3000/dict/logs" from=hello to=你好{}'.format(datetime.now().strftime('%Y%m%d%H%M%S')))
    local('http ":3000/dict/logs"')
