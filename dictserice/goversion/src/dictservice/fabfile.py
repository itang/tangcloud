# -*- coding: utf-8 -*-

from fabric.api import *


def run():
    local('GOPATH=$PWD/../.. go run dictservice.go')


def test():
    local('http ":9800/dict/logs"')
