# -*- coding: utf-8 -*-

from fabric.api import *
from datetime import datetime


def run():
    """run"""
    local('GOPATH=$PWD/../.. go run dictservice.go')


def test():
    """test"""
    local('http post ":9800/dict/logs" from=hello to="你好{}"'.format(datetime.now()))
    local('http ":9800/dict/logs"')


def bench():
    """bench"""
    local('wrk http://localhost:9800/dict/logs -d30 -c100 -t4')
