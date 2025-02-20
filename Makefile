.PHONY: all build update up down clean prune

all: build

NAME=stockinfo-backend
TAG=1.3.3

build:
	docker build \
	--build-arg GIT_HASH=$(shell git rev-parse HEAD) \
	-t ghcr.io/junekimdev/$(NAME):$(TAG) .

# This updates local repo
update:
	@if [ -d .git ];	then \
		git fetch --all \
		&& git reset --hard origin/master; \
	else \
		echo "Git repo does not exist. Clone it first."; \
	fi

up:
	@docker compose up -d \
	&& sleep 5 \
	&& docker logs -t --tail 5 $(NAME)-1

down:
	docker compose down

clean:
	docker rmi $(shell docker images -qf dangling=true)

prune:
	docker builder prune -f
