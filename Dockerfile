FROM node:lts-bookworm as builder

WORKDIR /app
COPY ./daocker-system /app

RUN yarn install && npm run build

EXPOSE 3000

ENTRYPOINT [ "yarn", "dev" ]




