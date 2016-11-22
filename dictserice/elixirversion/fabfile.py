  # -*- coding: utf-8 -*-

from fabric.api import *


def help():
    """help"""
    print('help')


def run():
    """run"""
    local('foreman start')


def check():
    """check"""
    local('mix dialyzer')
    local('mix credo --strict')
