aws ecr get-login --no-include-email --region eu-central-1 | /bin/bash
docker build -t coffee-shop-api .
docker tag coffee-shop-api:latest 726120415058.dkr.ecr.eu-central-1.amazonaws.com/coffee-shop-api:latest
docker push 726120415058.dkr.ecr.eu-central-1.amazonaws.com/coffee-shop-api:latest