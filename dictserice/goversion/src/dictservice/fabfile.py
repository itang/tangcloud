# -*- coding: utf-8 -*-

import os
from datetime import datetime
from fabric.api import local, run, put


def gocmd(cmd):
    """run go cmd in the custom gopath"""
    # cwd = os.getcwd()
    dir_path = os.path.dirname(os.path.realpath(__file__))
    gopath = os.path.normpath(os.path.join(dir_path, '../../'))
    local('GOPATH={} {}'.format(gopath, cmd))


def start():
    """start"""
    gocmd('go run dictservice.go')


def dev():
    """dev"""
    gocmd('realize run')


# def start():
#     """forego"""
#     # local('forego start') # https://github.com/ddollar/forego
#
#     local('honcho start') # https://github.com/nickstenning/honcho
#

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


def deploy():
    """deploy"""
    gocmd("go build")
    run('cd /data/gateway; docker-compose stop dict_go; rm -rf dict/goversion/go_echo_version')
    put('dictservice', '/data/gateway/dict/goversion/go_echo_version')
    run('cd /data/gateway/dict/goversion; chmod +x go_echo_version')
    run('cd /data/gateway; docker-compose start dict_go')
    local('rm dictservice')
