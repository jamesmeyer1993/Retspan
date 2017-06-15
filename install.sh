# install.sh - full installation of dependencies
#
# TODO: check for installed files so we can avoid this

# python pip

sudo apt-get install pip
sudo pip install --upgrade youtube-dl

# lltag

sudo apt-get install lltag

# mov retspan to /bin

sudo mv $pwd/bin/retspan /bin

echo "Installation Complete!"
