set -eux

# ascii codes for colored output
YELLOW="\e[93m"
RED="\e[31m"
END="\e[0m"

echo -e "$YELLOW""Checking docker version...$END"
DOCKER_VERSION=$(docker --version)

if [ -z "$DOCKER_VERSION" ]; then
    echo -e "$RED""You need to install docker!$END"
else
    echo -e "$YELLOW""Found '$DOCKER_VERSION'. Continue build.$END"
fi

BUILD_DIR="build_dir"
function build_single_version() {
    local VERSION_FEATURE_NAME=$1
    local NEEDED_FILE=$2
    local DOCKERFILE=$3
    local TAG=$4

    echo -e "$YELLOW""\nBuilding image for feature $VERSION_FEATURE_NAME, resulting file - $NEEDED_FILE.$END"
    docker build --build-arg VERSION_FEATURE_NAME=$VERSION_FEATURE_NAME --build-arg NEEDED_FILE=$NEEDED_FILE -f "$DOCKERFILE" -t libmysqlclient-bindings:$TAG .
    docker run --rm --entrypoint /bin/cat libmysqlclient-bindings:$TAG /$BUILD_DIR/$NEEDED_FILE > $PWD/src/$NEEDED_FILE
    echo -e "$YELLOW""File saved at $PWD/src/$NEEDED_FILE.$END\n"
}

build_single_version "mysql" "mysql.rs" "mysql.dockerfile" "mysql"
sleep 5
build_single_version "mariadb" "mariadb.rs" "mariadb.dockerfile" "mariadb"
sleep 3
echo -e "$YELLOW""All versions are built.$END"