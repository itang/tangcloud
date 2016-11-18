  # -*- coding: utf-8 -*-

from fabric.api import *


def help():
    """help"""
    print('help')


def run():
    """run"""
    local('foreman start')
