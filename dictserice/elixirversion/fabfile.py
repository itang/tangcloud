  # -*- coding: utf-8 -*-

from fabric.api import *


def help():
    """help"""
    print('help')


def start():
    """start"""
    local('foreman start')


def check():
    """check"""
    local('mix dialyzer')
    local('mix credo --strict')


def dist():
    """release"""
    local('mix clean')
    local('mix release.clean')
    #local('mix deps.get --only prod')
    local('MIX_ENV=prod mix compile')
    local('MIX_ENV=prod mix phoenix.digest')
    local('MIX_ENV=prod mix release --env=prod --verbose')


def deploy():
    """deploy"""
    dist()

    bin_file = 'rel/elixirversion/releases/0.0.1/elixirversion.tar.gz'
    local('du -sh {}'.format(bin_file))
    with cd('/data/tang/dict'):
        put(bin_file, '.')
        run('rm -rf dict/elixirversion')
        run('mkdir elixirversion')
        run('tar -zxf elixirversion.tar.gz -C elixirversion')

        run('supervisorctl restart dict_elixir')
