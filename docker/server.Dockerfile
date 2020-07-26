FROM node:12-alpine

RUN \
    echo "==> Install app..."                && \
    npm install -g --production json-server  && \
    \
    \
    echo "==> Remove unused temp..."         && \
    rm -rf /root/.npm                  \
           /usr/lib/node_modules/npm

VOLUME [ "/data" ]
WORKDIR /config

COPY ./server/ /config

# Always pass one arg at a time. Don't put anything seperated by spaces here
CMD ["json-server", "--config", "/config/config.json", "/config/db.json"]