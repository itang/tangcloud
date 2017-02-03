# -*- coding: utf-8 -*-

from fabric.api import *
from datetime import datetime
import os


def gocmd(cmd):
    """run go cmd in the custom gopath"""
    # cwd = os.getcwd()
    dir_path = os.path.dirname(os.path.realpath(__file__))
    gopath = os.path.normpath(os.path.join(dir_path, '../../'))
    local('GOPATH={} {}'.format(gopath, cmd))


def prepare():
    """prepare"""
    for p in ['github.com/itang/gotang', 'github.com/labstack/echo', 'github.com/uber-go/zap', 'gopkg.in/redis.v5']:
        gocmd('go get {}'.format(p))


def run():
    """run"""
    gocmd('go run dictservice.go')


def dev():
    """dev"""
    gocmd('realize fast')


def start():
    """forego"""
    # local('forego start') # https://github.com/ddollar/forego

    local('honcho start') # https://github.com/nickstenning/honcho


def repl():
    """repl"""
    local('gore')


def test():
    """test"""
    local('http post ":8080/api/dict/logs" from=hello to="你好{}"'.format(datetime.now()))
    local('http ":8080/api/dict/logs"')


def bench():
    """bench"""
    local('wrk http://localhost:8080/api/dict/logs -d30 -c100 -t4')


def dist():
    """dist for 386"""
    gocmd('gox -os="linux" -arch="386"')
    local('upx dictservice_linux_386')


def fmt():
    """go fmt ./..."""
    gocmd('go fmt ./...')


def update():
    """"dep ensure -update"""
    status()
    gocmd('dep ensure -update')
    status()


def status():
    """dep status"""
    gocmd('dep status')
