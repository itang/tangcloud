# -*- coding: utf-8 -*-

from fabric.api import *
from fabric.contrib.files import exists


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


dist_cmds = [  # 'mix deps.get',
    #'mix clean',
    #'mix release.clean',
    'mix deps.get --only prod',
    'mix phoenix.digest',
    'MIX_ENV=prod mix compile',
    #'MIX_ENV=prod mix release --env=prod --verbose'
]


def checkout():
    """checkout source"""
    with cd('/data/gateway/source'):
        if(exists('tangcloud')):
            with cd('tangcloud'):
                run('git pull')
        else:
            run('git clone git@github.com:itang/tangcloud.git')


def dist():
    """release"""
    with cd('/data/gateway/source/tangcloud/dictserice/elixirversion'):
        for cmd in dist_cmds:
            run(cmd)


def deploy():
    """deploy"""
    checkout()

    dist()

    restart_remote()


def __kill_by_name(name):
    with settings(warn_only=True):
        local("pkill {}".format(name))


def kill():
    """kill redis nginx"""
    for name in ['redis-server', 'nginx']:
        __kill_by_name(name)
