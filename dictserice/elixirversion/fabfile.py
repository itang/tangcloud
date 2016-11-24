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


def stop_remote():
    """restart remote"""
    with settings(warn_only=True):
        run('supervisorctl stop dict_elixir')


def start_remote():
    """restart remote"""
    run('supervisorctl start dict_elixir')


def restart_remote():
    """restart remote"""
    stop_remote()
    start_remote()


dist_cmds = ['mix clean', 'mix release.clean',
             'mix deps.get --only prod', 'MIX_ENV=prod mix compile',
             'MIX_ENV=prod mix release --env=prod --verbose']


def dist_by_local():
    """release"""
    for cmd in dist_cmds:
        local(cmd)


def dist_by_docker():
    """dist by docker"""
    local(
        'docker run -it -v "$PWD:/elixirversion" itang/test-erlang-i386 /bin/bash -c "mix local.hex --force;cd /elixirversion;{}"'.format(
            ';'.join(dist_cmds)))


def deploy(by='docker'):
    """deploy"""
    if by == 'docker':
        dist_by_docker()
    else:
        dist_by_local()

    bin_file = 'rel/elixirversion/releases/0.0.1/elixirversion.tar.gz'
    local('du -sh {}'.format(bin_file))
    with cd('/data/tang/dict'):
        put(bin_file, '.')
        run('rm -rf elixirversion')
        run('mkdir elixirversion')
        run('tar -zxf elixirversion.tar.gz -C elixirversion')

        restart_remote()
