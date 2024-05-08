# My Website

This is [https://s1n7ax.com](https://s1n7ax.com) built with Rust 🦀

## Development

- Clone the project
- Run the dev container `devcontainer up --workspace-folder .`
- Open up the shell `devcontainer exec --workspace-folder . bash`
- Install node dependencies `yarn install`
- Run tailwind watch process `yarn run tailwind:watch`
- Run leptos watch process `cargo leptos watch`
- Open the browser `http://localhost:3000`

## Deployment

Deployment is done using docker images.

Github workflow in this project, builds a new image and push it to docker hub.
For the deployment, SSH should be configured in the server.

Following secrets should be registered in the Github Actions

- `DOCKERHUB_USERNAME` - Name of the docker hub user
- `DOCKERHUB_TOKEN` - Access token
- `DO_IP` - IP of the server
- `DO_USERNAME` - Name of the user in the server
- `DO_SSH_PRIVATE_KEY` - SSH Private key

## SSL & TSL update

Using the `certbot` client, we can request a new certificate.

```shell
docker compose up certbot
```

Let's encrypt certificate will be outdated in 90 days. So another request has
to be made to update. This can be automated using a cron job
