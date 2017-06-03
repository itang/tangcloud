# -*- coding: utf-8 -*-

from fabric.api import *


def help():
    """help"""
    print('help')


def dev():
    """dev"""
    local('mix phx.server')


def check():
    """check"""
    local('mix dialyzer')
    local('mix credo --strict')


def restart_remote():
    """restart remote"""
    with settings(warn_only=True):
        with cd('/data/gateway'):
            run('docker-compose restart dict_elixir')


dist_cmds = ['mix clean',
             'mix release.clean',
             'mix deps.get --only prod',
             'MIX_ENV=prod mix compile',
             'MIX_ENV=prod mix release --env=prod --verbose']


def dist_by_local():
    """release"""
    for cmd in dist_cmds:
        local(cmd)


def deploy():
    """deploy"""
    dist_by_local()

    bin_file = 'rel/elixirversion/releases/0.0.1/elixirversion.tar.gz'
    local('du -sh {}'.format(bin_file))
    with cd('/data/gateway/dict'):
        put(bin_file, '.')
        run('rm -rf elixirversion')
        run('mkdir elixirversion')
        run('tar -zxf elixirversion.tar.gz -C elixirversion')

        restart_remote()


def __kill_by_name(name):
    with settings(warn_only=True):
        local("pkill {}".format(name))


def kill():
    """kill redis nginx"""
    for name in ['redis-server', 'nginx']:
        __kill_by_name(name)
