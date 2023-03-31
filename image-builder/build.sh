#!/bin/sh

title() {
    echo -e "\E[34m\n# $1\E[00m";
}

waitfor_image() {
    local uuid=$1

    local tstart=$(date +%s)
    echo "$(date +'%Y-%m-%d %H:%M:%S') STARTED"

    local status=$(sudo composer-cli compose status | grep ${uuid} | awk '{print $2}')
    while [ "${status}" = "RUNNING" -o "${status}" = "WAITING" ]; do
        sleep 10
        status=$(sudo composer-cli compose status | grep ${uuid} | awk '{print $2}')
        echo -en "$(date +'%Y-%m-%d %H:%M:%S') ${status}\r"
    done

    local tend=$(date +%s)
    echo "$(date +'%Y-%m-%d %H:%M:%S') ${status} - elapsed $(( (tend - tstart) / 60 )) minutes"

    if [ "${status}" = "FAILED" ]; then
        download_image ${uuid} 1
        echo "Blueprint build has failed. For more information, review the downloaded logs"
        exit 1
    fi
}

download_image() {
    local uuid=$1
    sudo composer-cli compose metadata ${uuid}
    sudo composer-cli compose image ${uuid}
   
    sudo chown -R $(whoami). "${uuid}*.tar"
}


title "Loading sources"
sudo composer-cli sources add firmware-repo.toml

title "Building ostree container image"
sudo composer-cli blueprints push embedded-firmware-bluerpint.toml
sudo composer-cli blueprints depsolve embedded-firmware
buildid=$(sudo composer-cli compose start embedded-firmware edge-container | awk '{print $2}')
waitfor_image "${buildid}"
download_image "${buildid}"

title "Starting container server"

imageid=$(cat ./${buildid}-container.tar | sudo podman load | grep -o -P '(?<=sha256[@:])[a-z0-9]*')

sudo podman tag ${imageid} localhost/firmware-edge-container:1.0.0
sudo podman run -d --name=firmware-edge-server -p 8080:8080 localhost/firmware-edge-container:1.0.0

# create the iso
sudo composer-cli blueprints push installer.yaml
sudo composer-cli blueprints depsolve embedded-firmware-installer
buildid=$(sudo composer-cli compose start-ostree --ref rhel/9/x86_64/edge --url http://192.168.1.10:8080/repo/ embedded-firmware-installer edge-installer |  awk '{print $2}')
waitfor_image "${buildid}"
download_image "${buildid}"
mkksiso --ks kickstart.ks "${buildid}.iso" embedded-firmware.iso

exit 0
# update
sudo composer-cli compose start-ostree --ref rhel/9/x86_64/edge --url http://192.168.1.10:8080/repo/ embedded-firmware edge-container

