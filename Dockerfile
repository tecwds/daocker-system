FROM m.daocloud.io/node as builder

WORKDIR /app

# 复制依赖项
COPY daocker-system/package.json .

# 安装
RUN yarn install --registry https://registry.npmmirror.com

FROM m.daocloud.io/node as runner

WORKDIR /app

COPY --from=builder /app .
COPY daocker-system .

EXPOSE 3000

ENTRYPOINT [ "yarn", "dev" ]




