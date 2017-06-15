# install.sh - full installation of dependencies
#
# TODO: check for installed files so we can avoid this

# python pip

sudo apt-get install python-pip
sudo pip install --upgrade youtube-dl

# lltag

sudo apt-get install lltag

# mov retspan to /bin

DIR=$(echo $pwd)

sudo mv $DIR/bin/retspan /bin
sudo chmod +x /bin/retspan

echo "Installation Complete!"
