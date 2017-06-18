# -*- coding: utf-8 -*-

from fabric.api import *


def help():
    """help"""
    print('help')


def dev():
    """dev"""
    local('watchexec -e rs -r "cargo run"')


def deploy():
    """run"""
    local('cargo build --release')
    run('cd /data/gateway; docker-compose stop dict; docker-compose ps')
    put('target/release/main',
        '/data/gateway/dict/rustversion/rust_rocket_version')
    run('cd /data/gateway; docker-compose restart dict; docker-compose ps')
