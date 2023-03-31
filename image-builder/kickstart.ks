lang en_US.UTF-8
keyboard us
timezone UTC
#text
# Reboot after installation
reboot

# Network information
network --bootproto=dhcp --device=link --activate --onboot=on

# OSTree setup
#ostreesetup --osname="rhel" --remote="edge" --url="http://192.168.1.10:8080/repo/" --ref="rhel/9/x86_64/edge" --nogpg

ostreesetup --nogpg --osname=rhel --remote=edge --url=file:///run/install/repo/ostree/repo --ref=rhel/9/x86_64/edge

ignoredisk --only-use=mmcblk0
bootloader --append="crashkernel=1G-4G:192M,4G-64G:256M,64G-:512M" --location=mbr --boot-drive=mmcblk0
autopart --type=plain --fstype=xfs --nohome
zerombr
clearpart --all --initlabel --drives=mmcblk0

rootpw --plaintext redhat

%post --logfile=/var/log/anaconda/post-user-install.log --erroronfail
# no sudo password for user
echo -e "majopela\tALL=(ALL)\tNOPASSWD: ALL" >> /etc/sudoers

# Replace the ostree server name
sed -i "/^url=/s/=.*/=http:\/\/192.168.1.10:8080\/repo\//" /etc/ostree/remotes.d/edge.conf

cat >/etc/udev/rules.d/00-device-names.rules <<EOF
KERNEL=="ttyACM[0-9]*", SUBSYSTEM=="tty", ATTRS{idVendor}=="2b23", ATTRS{idProduct}=="e011", SYMLINK+="display", MODE="0660", GROUP="dialout"
KERNEL=="ttyACM[0-9]*", SUBSYSTEM=="tty", ATTRS{idVendor}=="2b23", ATTRS{idProduct}=="e012", SYMLINK+="radar", MODE="0660", GROUP="dialout"
EOF

%end
