# -*- coding: utf-8 -*-

from fabric.api import *
from datetime import datetime


def run():
    """run"""
    local('GOPATH=$PWD/../.. go run dictservice.go')


def test():
    """test"""
    local('http post ":8080/api/dict/logs" from=hello to="你好{}"'.format(datetime.now()))
    local('http ":8080/api/dict/logs"')


def bench():
    """bench"""
    local('wrk http://localhost:8080/api/dict/logs -d30 -c100 -t4')


def dist():
    """dist for 386"""
    local('GOPATH=$PWD/../.. gox -os="linux" -arch="386"')
    local('upx dictservice_linux_386')
