Digitalocean Guide
==================

Maintaining the spec.yaml file is what is needed to deploy the app to digital ocean/
Digital Ocean’s App Platform uses a declarative configuration file to let us specify what our application
deployment should look like - they call it _App_ Spec.

To authenticate we need to go to https://cloud.digitalocean.com/account/api/tokens \
create an api token and enter it after this command:

`doctl auth init`

We will also need to connect the account to github (to give permissions to the repo) \
To deploy the app from the `spec.yaml` file

`doctl apps create --spec spec.yaml`

To see the status of the deployment and running application:

`doctl apps list`
