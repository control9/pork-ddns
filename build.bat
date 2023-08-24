cross build --release --target x86_64-unknown-linux-gnu

docker build -t pork-ddns:latest .
docker save  pork-ddns:latest > img.tar
