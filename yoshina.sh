#!/bin/bash

if [ $1 = "build" ]; then
    rm ./build -rf
    docker rm -f catsatmat-icfpc2020-local-judge-container 2> /dev/null 
    docker build . -t catsatmat-icfpc2020-local-judge-container
    exit 0
fi

if [ $1 = "run" ]; then
    if [ $# -ne 3 ]; then
        echo "./yoshina.sh build でビルド"
        echo "./yoshina.sh run \`serverUrl\` \`playerKey\` で実行"
        echo "runのときは引数がいるよ"
        exit 1
    fi

    docker run catsatmat-icfpc2020-local-judge-container $2 $3
    exit 0
fi


echo "./yoshina.sh build でビルド"
echo "./yoshina.sh run `serverUrl` `playerKey` で実行"
echo "それ以外は何もしないよ"

