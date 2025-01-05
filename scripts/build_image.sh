# Build the yocto docker image to run builds in
docker build -f release/Dockerfile -t yocto-release .

docker run -u $(id -u ${USER}):$(id -g ${USER}) -v /home/$USER:/home/$USER -w $(pwd) yocto-release release/build.sh