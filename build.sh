set -e
docker build --progress=plain -t quay.io/wutiarn/sms-gw-rust .
# docker push quay.io/wutiarn/sms-gw-rust