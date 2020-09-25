DC=docker-compose

.DEFAULT_GOAL := help

.PHONY: help

help:  ##Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

up: ##Start Docker
	$(DC) up -d

stop: ##Start Docker
	$(DC) stop

upb: ##Start Docker
	$(DC) up -d --build

do: ##Stop Docher
	$(DC) down

ex: ##Connect to Service
	$(DC) exec --user=www-data electron /bin/bash

urn: ##Generate URN validators
	$(DC) run --user=www-data --volume $(PWD)/.idea:/var/idea php /app/bin/magento dev:urn-catalog:generate /var/idea/misc.xml
	sed -i 's+/app/vendor/+$(PWD)/htdocs/vendor/+g' $(PWD)/.idea/misc.xml
	echo "About urn : https://devdocs.magento.com/guides/v2.3/config-guide/cli/config-cli-subcommands-urn.html"
