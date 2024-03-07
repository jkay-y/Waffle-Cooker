cd $(git rev-parse --show-toplevel)

if [ ! -d "build" ]; then
    mkdir build
fi

cd build

cmake ..
make
./WaffleCooker

cd $(git rev-parse --show-toplevel)

rm -rf build/