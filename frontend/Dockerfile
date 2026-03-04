FROM node:22-alpine

WORKDIR /app

RUN apk add --no-cache tini

COPY package.json package-lock.json* ./                                                                                                                     
RUN npm install

COPY . .

ENTRYPOINT ["/sbin/tini", "--"]

CMD ["npm", "run", "dev"]