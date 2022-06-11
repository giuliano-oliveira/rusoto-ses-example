.ONESHELL: # Enable cd to work properly

INFRASTRUCTURE_PATH = "./infrastructure"

define tf_deploy
cd $(INFRASTRUCTURE_PATH)
terraform workspace select $1
terraform apply --auto-approve
endef

lambda-build:
	cargo lambda build --release --output-format zip

tf-init:
	cd $(INFRASTRUCTURE_PATH)
	terraform init

test:
	cargo test --release

lambda-deploy-development: lambda-build tf-init
	$(call tf_deploy,development)
lambda-deploy-production: lambda-build tf-init
	$(call tf_deploy,default)
