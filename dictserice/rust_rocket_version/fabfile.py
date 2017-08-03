# -*- coding: utf-8 -*-

import fabric.api as fab


def help():
    """Help."""
    fab.local('fab -l')


def dev():
    """Run on dev."""
    fab.local('watchexec -e rs -r "cargo run"')


def deploy():
    """Deploy to cloud."""
    fab.local('cargo build --release')
    fab.run('cd /data/gateway; docker-compose stop dict; docker-compose ps')
    fab.put('target/release/rust_rocket_version', '/data/gateway/dict/rustversion/rust_rocket_version')
    fab.run('cd /data/gateway; docker-compose restart dict; docker-compose ps')
