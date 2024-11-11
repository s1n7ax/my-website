# My Website

This is [https://s1n7ax.com](https://s1n7ax.com) built with Rust 🦀

## Demo

https://github.com/user-attachments/assets/ff3c5a12-fb95-43d8-9154-fc804085a259

## Screenshots

![PC](https://github.com/user-attachments/assets/b57bf6aa-66cb-41f9-9da4-45a07a647e09)

![Mobile](https://github.com/user-attachments/assets/0209cdb6-4d6a-4ab4-8d64-2e396fd18d07)

## Development

- Clone the project
- Prepare the dev environment
  - Using direnv (recommended)
    - Allow the direnv to initiate the environment using `direnv allow`
  - Using Devcontainer
    - Run the dev container `devcontainer up --workspace-folder .`
    - Open up the shell `devcontainer exec --workspace-folder . bash`
- Install node dependencies `yarn install`
- Run tailwind watch process `yarn dev`
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

## SSL & TLS

Using the `certbot` client, we can request a new certificate.

```shell
docker compose up certbot
```

Let's encrypt certificate will be outdated in 90 days. So another request has
to be made to update. This can be automated using a cron job

### Automated SSL renewal

[This documentation](https://eff-certbot.readthedocs.io/en/latest/using.html#setting-up-automated-renewal)
shows how to add a crone job to automatically renew the certificate
