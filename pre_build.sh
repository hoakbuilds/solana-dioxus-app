if test -d "unzipped"; then
    echo "Already unzipped charting library.."
else
    echo "Unzipping charting library.." &&
    mkdir unzipped && 
    tar -xzC unzipped -f vendor/charting_library.tar.gz &&
    echo "Charting library unzipped!"
fi 
