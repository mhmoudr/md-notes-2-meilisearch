docker run -it --rm \
  -p 7700:7700 \
  -v $(pwd)/meili_data:/meili_data \
  -e MEILI_MASTER_KEY='dZzLAoGuTcErWP6o6rbDmJWOXbdYnxIs9_iD6Z__1T4' \
  getmeili/meilisearch