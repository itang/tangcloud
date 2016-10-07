# -*- coding: utf-8 -*-

from fabric.api import *


def help():
    """help"""
    print('help')


def run():
    """run"""
    local('vapor build && vapor run serve')
