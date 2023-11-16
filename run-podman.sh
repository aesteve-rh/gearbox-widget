#!/usr/bin/env bash

PARENT_PATH=$(cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P)
REMOTE_PORT=33452

function print_usage {
    echo "Usage: $0 CMD [OPTIONS]"
    echo "Commands:"
    echo "  build                    Build the container"
    echo "  server                   Restart the adb server with nodaemon"
    echo "  run                      Run the container with options"
    echo "                 '-r username'  Run using remote container at quay.io/username [default:localhost]"
    echo "Options:"
    echo "  --h,--help               Show this help and exit"
    exit -1
}

if [ -z "$1" ]; then
    echo "Command missing"
    print_usage
    exit 1
fi

case "${1}" in
    -h | --help)
        print_usage
        ;;
    build)
        podman build -t gearbox-widget:latest -f container/Containerfile .
        ;;
    server)
        # Kill and (re-)start the server to listen in all interfaces
        ${PARENT_PATH}/container/bin/adb kill-server
        ${PARENT_PATH}/container/bin/adb -a nodaemon server start &> /dev/null &
        ;;
    run)
        OPTIND=2
        SERVER=localhost
        while getopts ":r:" opt; do
            case "${opt}" in
                r)
                    SERVER="quay.io/$OPTARG"
                    ;;
                *)
                    print_usage
                    ;;
            esac
        done
        shift $((OPTIND-1))
        LOCAL_PORT=`${PARENT_PATH}/container/bin/adb forward tcp:0 tcp:$REMOTE_PORT`
        podman run -ti --privileged --rm \
            --net host --userns keep-id \
            -e XDG_RUNTIME_DIR=/run/user/1000 \
            -e WAYLAND_DISPLAY=$WAYLAND_DISPLAY \
            -e RUST_BACKTRACE=full \
            -e ADB_PATH=/usr/bin/ \
            -e PULSE_SERVER=/run/user/1000/pulse/native \
            -v /run/user/1000/:/run/user/1000/:rw \
            ${SERVER}/gearbox-widget --local-port $LOCAL_PORT
        ;;
esac
