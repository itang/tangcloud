# -*- coding: utf-8 -*-

from fabric.api import *


def test():
    local('http post ":3000/dict/history" fromLang=en from=hello toLang=zh to=你好')
    local('http get ":3000/dict/history"')
