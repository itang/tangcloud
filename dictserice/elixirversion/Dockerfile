FROM elixir

RUN mix local.hex --force && \
    mix local.rebar --force

WORKDIR /app
EXPOSE 9902
ENV PORT=9902 MIX_ENV=prod REPLACE_OS_VARS=true SHELL=/bin/bash

# more link: https://robots.thoughtbot.com/deploying-elixir-to-aws-elastic-beanstalk-with-docker
