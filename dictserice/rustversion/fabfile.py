# .*. encoding: utf-8 .*.

from datetime import datetime
from fabric.api import *


def test():
    """test"""
    local('http post ":3000/dict/logs" from=hello to=你好{}'.format(datetime.now().strftime('%Y%m%d%H%M%S')))
    local('http ":3000/dict/logs"')


def update():
    """update"""
    local('cargo outdated && cargo update && cargo outdated')


def run():
    """run"""
    import subprocess
    process = subprocess.Popen('cd ../env;redis-server redis.conf', shell=True, stdout=subprocess.PIPE)
    # process.wait()
    local('cargo run')


# TODO: i386 target
def dist_i386():
    """dist for i368"""
    # ref link: https://github.com/rust-lang-nursery/rustup.rs#working-with-custom-toolchains-and-local-builds
    print('INFO: ' + '$ rustup target add i686-unknown-linux-musl')

    # issues link: https://github.com/rust-cn/rust_lang_cn/issues/7
    ##oi = '/usr/include/openssl'
    local('OPENSSL_STATIC=true cargo build --release --target i686-unknown-linux-musl')
