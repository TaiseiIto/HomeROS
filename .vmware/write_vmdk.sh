#!/bin/bash
# Usage: $ ./write_vmdk.sh ~/vmware/HomeROS/HomeROS.vmx
# TODO Embed this into xtask.

pushd $(dirname $0)
vmx=$1
virtual_machine_directory=$(dirname $vmx)
vmdk=$virtual_machine_directory/$(sed -n "s/^.*\s*=\s*\"\(.*\.vmdk\)\"$/\1/p" $vmx)
vhd=$(echo $vmdk | sed "s/vmdk$/vhd/")
nbd=/dev/nbd0
destination_path=destination
source_path=../target/release/x64
media_size=64M
sudo modprobe nbd max_part=16
qemu-img create -f vpc $vhd $media_size
sudo qemu-nbd --format=vpc --connect=$nbd $vhd
while [ $(sudo blockdev --getsize64 $nbd) -eq 0 ]; do
	sleep 0.1
done
sudo mkfs.vfat -v -c -F 32 $nbd
mkdir $destination_path
sudo mount $nbd $destination_path
sudo cp -r $source_path/* $destination_path
sudo umount $destination_path
sudo rm -rf $destination_path
sudo qemu-nbd --disconnect $nbd
qemu-img convert -f vpc -O vmdk $vhd $vmdk
popd
