# build the site
ddoc

# copy the site to the deployement stage
cp -r site/* ~/dev/www/dystroy/ddoc-columns/

# deploy on dystroy.org
~/dev/www/dystroy/deploy.sh
