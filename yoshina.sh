#!/bin/bash

if [ $# -ne 1 ]; then
    echo "./yoshina.sh build でビルド"
    echo "./yoshina.sh run で実行"
    exit 1
fi

if [ $1 = "build" ]; then
    rm ./build -rf
    docker rm -f catsatmat-icfpc2020-local-judge-container 2> /dev/null 
    docker build . -t catsatmat-icfpc2020-local-judge-container
    exit 0
fi

if [ $1 = "run" ]; then
    docker run catsatmat-icfpc2020-local-judge-container
    exit 0
fi


echo "./yoshina.sh build でビルド"
echo "./yoshina.sh run で実行"
echo "それ以外は何もしないよ"

