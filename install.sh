# Removing Old Version of Peaches
rm -rf ~/.local/bin/git-sync

# Get Latest Release from Github
wget https://github.com/KCaverly/git-sync/releases/download/v0.1.0/git-sync --no-check-certificate -P ~/.local/bin/
chmod +x ~/.local/bin/git-sync

