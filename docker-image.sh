#!/usr/bin/env bash 

# Version 3.2.1, 2020-08-10

# Change log:

# 3.2.1, 2020-08-10
#    - sed macOS adopted
#    - fixed sed for docker ARG
#    - shellcheck adopted
# 3.2.0, 2020-07-27
#    - added "' || exit $?" at 114 line

# env_src=(.env prod/.env)
env_src=(.env)
cmdname=${0##*/}

echoerr() { 
    echo "ERR: $*" 1>&2 
}

usage()
{
    cat << USAGE >&2
Description:
    build image of service (Dockerfile in subfolder at ./TARGET/) and push to Docker Hub
Usage:
    $cmdname [--help | [SERVICE] [-b] ]
params:
    TARGET      'dev' or 'prod', 'prod' by default
    SERVICE     from self-titled subfolder at ./TARGET/, 'proj' by default
options:
    -b          build only, no push
    --help      this help
    --no-cache  pass '--no-cache' to 'docker build'
USAGE
    exit 1
}

build_only=
service=
opts=()

# process arguments
while [[ $# -gt 0 ]]
do
    case "$1" in
        -b)
            build_only=1
            shift 1
        ;;

        --help)
            usage
        ;;

        --no-cache )
            opts+=("$1")
            shift 1
        ;;

        -* )
            echoerr "Unknown option: $1"
            usage
        ;;

        dev | prod) 
            if [[ $target == "" ]]; then
                target=$1
            else 
                echoerr "TARGET must be specified only once, but two found: '$target' and '$1'"
                usage
            fi
            shift 1
        ;;

        *)
            if [[ $service == "" ]]; then
                service=$1
            else 
                echoerr "SERVICE must be specified only once, but two found: '$service' and '$1'"
                usage
            fi
            shift 1
        ;;
    esac
done

if [[ ! $target ]]; then
    target=prod
fi
TARGET=${target^^}

if [[ ! $service ]]; then
    service=proj
fi
SERVICE=${service^^}

mapfile -t envs < <( cat "${env_src[@]}" | grep -v '#' | grep -v '^[[:space:]]*$' )
# shellcheck disable=SC2016
bw_version=$(env "${envs[@]}" bash -c 'echo $BW_'$TARGET'_'$SERVICE'_VERSION')
if [[ ! $bw_version ]]; then
    echoerr "BW_${TARGET}_${SERVICE}_VERSION value must be specified in ${env_src[*]}"
    exit 1
fi

version_fspec="$target/ver.yml"
did_version=$(sed -nE "s/^$service:[[:space:]]*([^[:space:]])/\\1/p;" "$version_fspec")

if [[ ! $did_version ]]; then
    echoerr "$service: VERSION not found in file'$version_fspec'"
    exit 1
fi

if [[ ! $build_only && "$bw_version" == "$did_version" ]]; then
    echoerr "BW_${TARGET}_${SERVICE}_VERSION ($bw_version) in ${env_src[*]} must differ (be bigger) than version ($did_version) in file'$version_fspec' in line: $service: $did_version"
    exit 1
fi

if [[ -e "$target/$service/before.sh" ]]; then
    env "${envs[@]}" bash -c '
        "'"$target/$service/"'before.sh"
    ' || exit $?
fi

# shellcheck disable=SC2016,SC1004
env "${envs[@]}" bash -c '
    tag="bazawinner/'"$target"'-$BW_PROJ_NAME-'"$service"':$BW_'"$TARGET"'_'"$SERVICE"'_VERSION"
    echo Building $tag . . .
    echo docker build '"${opts[*]}"' \
        $(
            src="'"$target"'/'"$service"'/Dockerfile"; 
            cat $src | 
            sed -nE "s/^[[:space:]]*ARG[[:space:]]+([[:alnum:]_]+)/--build-arg=\1/p"
        ) \
        -t $tag \
        '"$target"'/'"$service"'
    docker build '"${opts[*]}"' \
        $(
            src="'"$target"'/'"$service"'/Dockerfile"; 
            cat $src | 
            sed -nE "s/^[[:space:]]*ARG[[:space:]]+([[:alnum:]_]+)([^=]|$)/--build-arg=\1/p"
        ) \
        -t $tag \
        -f '"$target"'/'"$service"'/Dockerfile . && \
    echo OK: Built $tag && \
    if [[ ! "'"$build_only"'" ]] 
    then
        echo Pushing $tag . . .
        docker push $tag
        echo OK: Pushed $tag
    fi
'
