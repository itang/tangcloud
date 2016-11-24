#FROM ioft/i386-ubuntu

## https://hub.docker.com/r/i386/erlang/
FROM i386/erlang

#COPY esl-erlang_19.1.5-1~ubuntu~trusty_i386.deb .
COPY Precompiled.zip .

#RUN apt-get install -y --force-yes wget
## http://mirrors.163.com/.help/ubuntu.html
RUN wget http://mirrors.163.com/.help/sources.list.trusty
RUN mv sources.list.trusty /etc/apt/sources.list

#RUN "wget -c -O- http://packages.erlang-solutions.com/ubuntu/erlang_solutions.asc | apt-key add -"
#RUN 'echo "deb http://packages.erlang-solutions.com/ubuntu trusty contrib" | tee -a /etc/apt/sources.list.d/erlang_solutions.list > /dev/null'
RUN apt-get update

#RUN wget https://packages.erlang-solutions.com/erlang-solutions_1.0_all.deb
#RUN dpkg -i erlang-solutions_1.0_all.deb

RUN apt-get install -y --force-yes unzip
#RUN apt-get install -y --force-yes python-urllib3 python-html5lib python-six python-requests python-pip

#RUN apt-get install -y --force-yes -f libpango-1.0 libgtk2.0 libwxgtk3.0  libwxbase3.0-0
#RUN apt-get install -y --force-yes esl-erlang
#RUN apt-get install -y --force-yes -f libpango-1.0 libgtk2.0 libwxgtk3.0  libwxbase3.0-0
#RUN dpkg -i esl-erlang_19.1.5-1~ubuntu~trusty_i386.deb
RUN unzip Precompiled.zip -d elixir

ENV PATH="/elixir/bin:${PATH}"


#RUN pip install fabric
