#!/bin/bash

if [ $1 = "build" ]; then
    rm ./build -rf
    docker rm -f catsatmat-icfpc2020-local-judge-container 2> /dev/null 
    docker build . -t catsatmat-icfpc2020-local-judge-container
    exit 0
fi

if [ $1 = "run" ]; then
    if [ $# -ne 3 ]; then
        docker run catsatmat-icfpc2020-local-judge-container $2 $3
        exit 0
    fi
    if [ $# -ne 1 ]; then
        docker run catsatmat-icfpc2020-local-judge-container
        exit 0
    fi
fi


echo "./yoshina.sh build でビルド"
echo "./yoshina.sh run  あるいは"
echo "./yoshina.sh run `serverUrl` `playerKey` で実行"
echo "それ以外は何もしないよ"

