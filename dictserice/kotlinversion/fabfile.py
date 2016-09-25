# -*- coding: utf-8 -*-

from fabric.api import *


def test():
    local('http post ":8080/dict/logs" fromLang=en from=hello toLang=zh to=你好')
    local('http get ":8080/dict/logs"')
